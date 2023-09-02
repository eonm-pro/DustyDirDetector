use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

pub struct InactiveDirFinder {
    base_dir: PathBuf,
}

impl InactiveDirFinder {
    pub fn new(path: PathBuf) -> Self {
        Self { base_dir: path }
    }

    pub fn find(
        &self,
        max_inactivity_duration: chrono::Duration,
    ) -> Result<Vec<InactiveDir>, Box<dyn std::error::Error>> {
        let now = std::time::SystemTime::now();
        let mut root_dir = PathBuf::new();
        let mut result = HashMap::<PathBuf, Dir>::new();

        let walker = WalkDir::new(&self.base_dir)
            .follow_links(false)
            .same_file_system(true)
            .contents_first(false);

        for entry in walker
            .into_iter()
            .filter_entry(|entry| entry.file_type().is_dir())
            .flatten()
        {
            if entry.depth() == 0 {
                continue;
            }

            if entry.depth() == 1 {
                root_dir = entry.path().to_path_buf();
            }

            let last_modification = entry.metadata()?.modified()?;
            let inactivity_duration: chrono::Duration =
                chrono::Duration::from_std(now.duration_since(last_modification).or_else(
                    |e| Ok::<std::time::Duration, Box<dyn std::error::Error>>(e.duration()),
                )?)?;

            if inactivity_duration > max_inactivity_duration {
                result
                    .entry(root_dir.clone())
                    .and_modify(|dir| match dir {
                        Dir::InactiveDir(inactive_dir) => {
                            // we keep the lowest inactivity duration among sub dirs
                            if inactivity_duration < inactive_dir.inactive_since {
                                inactive_dir.inactive_since = inactivity_duration;
                            }
                        }
                        // if the dir is already active we dont update it to inactive
                        Dir::ActiveDir => (),
                    })
                    .or_insert(Dir::InactiveDir(InactiveDir {
                        path: root_dir.clone(),
                        inactive_since: inactivity_duration,
                    }));
            } else {
                result
                    .entry(root_dir.clone())
                    .and_modify(|dir| {
                        *dir = Dir::ActiveDir;
                    })
                    .or_insert(Dir::ActiveDir);
            }
        }

        let mut result_vec: Vec<InactiveDir> = result
            .into_values()
            .filter_map(|dir| match dir {
                Dir::InactiveDir(dir) => Some(dir),
                Dir::ActiveDir => None,
            })
            .collect();
        result_vec.sort_by(|a, b| b.cmp(a));

        Ok(result_vec)
    }
}

pub enum Dir {
    InactiveDir(InactiveDir),
    ActiveDir,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
/// Represents an inactive dir with a path and a inactive duration
pub struct InactiveDir {
    pub path: PathBuf,
    pub inactive_since: chrono::Duration,
}

impl Ord for InactiveDir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inactive_since.cmp(&other.inactive_since)
    }
}
