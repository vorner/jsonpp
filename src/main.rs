extern crate failure;
extern crate serde_json;

use std::io::{self, BufRead};
use std::process;

use failure::Error;
use serde_json::Value;

fn run() -> Result<(), Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let json = serde_json::from_str::<Value>(&line)?;
        serde_json::to_writer_pretty(io::stdout(), &json)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
