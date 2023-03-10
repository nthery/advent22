use std::cmp::{self, Ordering};
use std::io::{BufRead};

use anyhow::{bail};

use util::{self, Half, Parser};

fn main() -> anyhow::Result<()> {
    util::driver(2, DayTwoParser{})
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

// Second input field.
#[derive(Debug)]
enum CipherMove {
    X,
    Y,
    Z,
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

struct DayTwoParser;

impl Parser for DayTwoParser {
    fn parse(&mut self, half: Half, reader: &mut impl BufRead) -> anyhow::Result<u32> {
        let mut total_score = 0u32;
        for maybe_line in reader.lines() {
            let line = maybe_line?;
            let round = parse_round(&line, half)?;
            total_score += round.score();
        }
        Ok(total_score)
    }
}

fn parse_round(line: &str, half: Half) -> anyhow::Result<Round> {
    let mut words = line.split(' ');
    let elf_move = match words.next() {
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
        elf: elf_move,
        me: decrypt_move(half, elf_move, cipher),
    })
}

fn decrypt_move(half: Half, elf_move: Move, cipher: CipherMove) -> Move {
    match half {
        Half::First => match cipher {
            CipherMove::X => Move::Rock,
            CipherMove::Y => Move::Paper,
            CipherMove::Z => Move::Scissors,
        },
        Half::Second => match cipher {
            CipherMove::X => match elf_move {
                Move::Rock => Move::Scissors,
                Move::Scissors => Move::Paper,
                Move::Paper => Move::Rock,
            },
            CipherMove::Y => elf_move,
            CipherMove::Z => match elf_move {
                Move::Rock => Move::Paper,
                Move::Scissors => Move::Rock,
                Move::Paper => Move::Scissors,
            },
        },
    }
}
