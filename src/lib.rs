use anyhow::Context;
use clap::Parser;
use std::{thread, time::Duration};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,
}


pub fn do_hard_work() {
    thread::sleep(Duration::from_secs(1));
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
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