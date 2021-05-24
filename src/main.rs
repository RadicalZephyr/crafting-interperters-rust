use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    script_file: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    if let Some(script_file) = args.script_file {
        run_file(script_file)
    } else {
        run_prompt()
    }
}

fn run_prompt() -> anyhow::Result<()> {
    let stdin = io::stdin();
    run_all(stdin.lock().lines()).context("from standard input")
}

fn run_file(script: PathBuf) -> anyhow::Result<()> {
    let buf_reader = BufReader::new(File::open(script).context("")?);
    run_all(buf_reader.lines()).context("from file")
}

fn run_all(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<()> {
    for line_res in lines {
        let line = line_res.context("failed to read line")?;
        run(line)?;
    }
    Ok(())
}

fn run(_code: String) -> anyhow::Result<()> {
    todo!("actually run")
}
