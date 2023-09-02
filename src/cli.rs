use clap::Parser;
use humantime::parse_duration;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets the paths to analyze
    #[arg(short, long, value_name = "PATHS", required = true)]
    pub paths: Vec<PathBuf>,
    /// Sets the inactivity duration threshold
    #[arg(short, long, value_name = "DURATION", value_parser = clap::value_parser!(DurationParser))]
    pub inactive_for: DurationParser,
}

#[derive(Clone)]
pub struct DurationParser(pub chrono::Duration);

impl From<String> for DurationParser {
    fn from(value: String) -> Self {
        DurationParser(
            chrono::Duration::from_std(parse_duration(&value).expect("Failed to parse duration"))
                .expect("Invalid duration"),
        )
    }
}
