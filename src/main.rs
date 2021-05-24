use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use structopt::StructOpt;
use thiserror::Error;

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

#[derive(Debug, Error)]
#[error("[line {line_number}] Error{location}: {error}")]
struct Error {
    line_number: usize,
    location: String,
    error: anyhow::Error,
}

type Result<T> = std::result::Result<T, Error>;

fn run_prompt() -> anyhow::Result<()> {
    let stdin = io::stdin();
    loop {
        let result = run_all(stdin.lock().lines()).context("from standard input");
        print_interpreter_error(result)?;
    }
}

fn run_file(script: PathBuf) -> anyhow::Result<()> {
    let buf_reader = BufReader::new(File::open(script).context("")?);
    let result = run_all(buf_reader.lines());
    print_interpreter_error(result).context("from file")
}

fn print_interpreter_error(error: anyhow::Result<Result<()>>) -> anyhow::Result<()> {
    match error {
        Ok(Ok(())) => Ok(()),
        Ok(Err(interpreter_error)) => {
            eprintln!("{}", interpreter_error);
            Ok(())
        }
        Err(io_error) => Err(io_error)?,
    }
}

fn run_all(lines: impl Iterator<Item = io::Result<String>>) -> anyhow::Result<Result<()>> {
    for line_res in lines {
        let line = line_res.context("failed to read line")?;
        if let Err(interpreter_error) = run(line) {
            return Ok(Err(interpreter_error));
        }
    }
    Ok(Ok(()))
}

fn run(_code: String) -> Result<()> {
    todo!("actually run")
}
