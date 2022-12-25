use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        anyhow::bail!("missing input file");
    }

    let file = File::open(&args[1]).context(format!("cannot open {}", args[1]))?;
    let mut file_reader = BufReader::new(file);

    let nb_calories = parse_top_3_packs(&mut file_reader)?;
    println!("Biggest pack: {} calories", nb_calories);

    Ok(())
}

// Return the total number of calories in the the first 3 backpacks by descending order.
fn parse_top_3_packs<T: BufRead>(reader: &mut T) -> anyhow::Result<u32> {
    let packs = packs(reader);
    let mut calories = packs.collect::<anyhow::Result<Vec<u32>>>()?;
    calories.sort_by(|l, r| r.cmp(l));
    Ok(calories.iter().take(3).sum())
}

// Return an iterator over the calories contained in the backpacks of each elf.
fn packs<B: BufRead>(reader: B) -> Packs<B> {
    Packs {
        lines: reader.lines(),
    }
}

struct Packs<B: BufRead> {
    lines: Lines<B>,
}

impl<B: BufRead> Iterator for Packs<B> {
    type Item = Result<u32, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = 0u32;
        for maybe_line in &mut self.lines {
            let line = match maybe_line {
                Ok(payload) => payload,
                Err(e) => return Some(Err(e.into())),
            };
            if line.is_empty() {
                break;
            }
            match line.parse::<u32>() {
                Ok(n) => total += n,
                Err(e) => return Some(Err(e.into())),
            }
        }

        if total > 0 {
            Some(Ok(total))
        } else {
            None
        }
    }
}
