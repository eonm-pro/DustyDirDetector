mod cli;
mod inactive_dir;

use clap::Parser;
use inactive_dir::InactiveDirFinder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let paths = cli.paths;
    let duration = cli.inactive_for.0;

    for path in paths {
        for dir in InactiveDirFinder::new(path).find(duration)? {
            if atty::isnt(atty::Stream::Stdout) {
                println!(
                    "{}\t{}",
                    dir.inactive_since.to_std()?.as_nanos(),
                    std::fs::canonicalize(dir.path)?.to_string_lossy(),
                )
            } else {
                println!(
                    "{} is untouched for {}",
                    dir.path.to_string_lossy(),
                    humantime::format_duration(dir.inactive_since.to_std()?)
                )
            }
        }
    }

    Ok(())
}
