use anyhow::Context;
use clap::Parser;
use std::{fs, thread, time::Duration};

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
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    let pb = indicatif::ProgressBar::new(5);
    for i in 0..5 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    Ok(())
}


fn do_hard_work() {
    thread::sleep(Duration::from_secs(1));
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[allow(unused)]
fn answer() -> i8 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}