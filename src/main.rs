use anyhow::Context;
use clap::Parser;
use grrs::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
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
