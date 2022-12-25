use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        anyhow::bail!("missing input file");
    }

    let file = File::open(&args[1]).context(format!("cannot open {}", args[1]))?;
    let mut file_reader = BufReader::new(file);

    let nb_calories = parse_total_food(&mut file_reader)?;
    println!("Biggest pack: {} calories", nb_calories);

    Ok(())
}

fn parse_total_food<T: BufRead>(reader: &mut T) -> anyhow::Result<u32> {
    let mut biggest_meal = 0u32;
    loop {
        let nb_calories = parse_elf_food(reader)?;
        if nb_calories == 0 {
            break;
        }
        biggest_meal = max(biggest_meal, nb_calories);
    }

    Ok(biggest_meal)
}

fn parse_elf_food<T: BufRead>(reader: &mut T) -> anyhow::Result<u32> {
    let mut total = 0u32;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let nb_calories: u32 = line.parse()?;
        total += nb_calories;
    }

    Ok(total)
}
