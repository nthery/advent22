//! Utilities shared by several advant solvers.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context};

// Solving first or second half?
#[derive(Debug, Clone, Copy)]
pub enum Half {
    First,
    Second,
}

pub trait Parser {
    fn parse(&mut self, half: Half, reader: &mut impl BufRead) -> anyhow::Result<u32>;
}

pub fn driver<P>(day: u32, mut parser: P) -> anyhow::Result<()>
where
    P: Parser,
 {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        anyhow::bail!(format!("usage: day{} 1|2 input_file", day));
    }

    let half = match args[1].as_str() {
        "1" => Half::First,
        "2" => Half::Second,
        _ => bail!("bad half"),
    };

    let file = File::open(&args[2]).context(format!("cannot open {}", args[1]))?;
    let mut file_reader = BufReader::new(file);

    let result = parser.parse(half, &mut file_reader)?;
    println!("Result: {result}");

    Ok(())
}
