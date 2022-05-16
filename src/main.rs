use anyhow::Context;
use clap::Parser;
use std::fs;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // let content = fs::read_to_string(&args.path).expect("could not read file");
    let content = fs::read_to_string(&args.path).with_context(|| {
        format!(
            "could not read file `{}`",
            &args.path.to_str().unwrap()
        )
    })?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
