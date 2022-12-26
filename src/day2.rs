use std::cmp::{self, Ordering};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context};

// Solving first or second half?
#[derive(Debug, Clone, Copy)]
enum Half {
    First,
    Second,
}

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        anyhow::bail!("usage: day2 1|2 input_file");
    }

    let half = match args[1].as_str() {
        "1" => Half::First,
        "2" => Half::Second,
        _ => bail!("bad half"),
    };

    let file = File::open(&args[2]).context(format!("cannot open {}", args[1]))?;
    let mut file_reader = BufReader::new(file);

    let score = parse_strategy(&mut file_reader, half)?;
    println!("Score: {score}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Scissors,
    Paper,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

// Second input field.
#[derive(Debug)]
enum CipherMove {
    X,
    Y,
    Z,
}

impl cmp::PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let res = match (self, other) {
            (Move::Rock, Move::Rock) => Ordering::Equal,
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (Move::Scissors, Move::Scissors) => Ordering::Equal,
            (Move::Scissors, Move::Paper) => Ordering::Greater,
            (Move::Paper, Move::Rock) => Ordering::Greater,
            (Move::Paper, Move::Scissors) => Ordering::Less,
            (Move::Paper, Move::Paper) => Ordering::Equal,
        };
        Some(res)
    }
}

impl cmp::Ord for Move {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Round {
    elf: Move,
    me: Move,
}

impl Round {
    fn score(&self) -> u32 {
        let bonus = match self.me.cmp(&self.elf) {
            cmp::Ordering::Less => 0,
            cmp::Ordering::Equal => 3,
            cmp::Ordering::Greater => 6,
        };

        self.me.score() + bonus
    }
}

fn parse_strategy(reader: &mut impl BufRead, half: Half) -> anyhow::Result<u32> {
    let mut total_score = 0u32;
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        let round = parse_round(&line, half)?;
        total_score += round.score();
    }
    Ok(total_score)
}

fn parse_round(line: &str, half: Half) -> anyhow::Result<Round> {
    let mut words = line.split(' ');
    let opponent_move = match words.next() {
        Some("A") => Move::Rock,
        Some("B") => Move::Paper,
        Some("C") => Move::Scissors,
        Some(m) => bail!(format!("unknown opponent move: {}", m)),
        None => bail!("missing opponent move"),
    };
    let cipher = match words.next() {
        Some("X") => CipherMove::X,
        Some("Y") => CipherMove::Y,
        Some("Z") => CipherMove::Z,
        Some(m) => bail!(format!("unknown cipher move: {}", m)),
        None => bail!("missing cipher move"),
    };
    if let Some(x) = words.next() {
        bail!("spurious field: {x}")
    }

    Ok(Round {
        elf: opponent_move,
        me: decrypt_move(half, cipher),
    })
}

fn decrypt_move(_half: Half, cipher: CipherMove) -> Move {
    match cipher {
        CipherMove::X => Move::Rock,
        CipherMove::Y => Move::Paper,
        CipherMove::Z => Move::Scissors,
    }
}
