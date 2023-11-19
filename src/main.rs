mod cli;
mod inactive_dir;

use clap::Parser;
use inactive_dir::InactiveDirFinder;

use owo_colors::OwoColorize;

use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Format, Modify, Style},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let paths = cli.paths;
    let duration = cli.inactive_for.0;
    let inactivity_type = cli.r#type;

    let mut table_builder = Builder::default();

    for path in paths {
        for dir in InactiveDirFinder::new(path).find(duration, &inactivity_type)? {
            if atty::isnt(atty::Stream::Stdout) {
                println!(
                    "{}\t{}",
                    dir.inactive_since.to_std()?.as_nanos(),
                    std::fs::canonicalize(dir.path)?.to_string_lossy(),
                )
            } else {
                table_builder.push_record(vec![
                    dir.path.to_string_lossy().to_string(),
                    humantime::format_duration(dir.inactive_since.to_std()?).to_string(),
                ]);
            }
        }
    }

    if atty::is(atty::Stream::Stdout) {
        table_builder.set_header(["path", "since"]);

        let table_builder = table_builder.index();

        let mut table = table_builder.build();
        table.with(Style::rounded()).with(
            Modify::new(Rows::first())
                .with(Alignment::center())
                .with(Format::content(|s| s.green().bold().to_string())),
        );

        println!("{}", table);
    }

    Ok(())
}
