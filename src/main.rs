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

#[allow(unused)]
fn answer() -> i8 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}