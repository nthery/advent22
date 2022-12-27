use std::io::BufRead;
use std::io::Lines;

use anyhow::bail;

use util::{self, Half, Parser};

fn main() -> anyhow::Result<()> {
    util::driver(3, DayThreeParser {})
}

struct DayThreeParser;

impl Parser for DayThreeParser {
    fn parse(&mut self, half: Half, reader: &mut impl BufRead) -> anyhow::Result<u32> {
        match half {
            Half::First => solve_first_half(reader),
            Half::Second => solve_second_half(reader),
        }
    }
}

fn solve_first_half(reader: &mut impl BufRead) -> anyhow::Result<u32> {
    let mut total_score = 0u32;
    for maybe_line in reader.lines() {
        let line = maybe_line?;
        total_score += find_duplicated_item(&line)?;
    }
    Ok(total_score)
}

fn find_duplicated_item(line: &str) -> anyhow::Result<u32> {
    if line.len() % 2 != 0 {
        bail!(format!("odd number of characters: {line}"))
    }
    let midpoint = line.len() / 2;
    let lhs = encode(&line[0..midpoint])?;
    let rhs = encode(&line[midpoint..])?;
    let dup_index = lhs & rhs;
    if dup_index.count_ones() != 1 {
        bail!(format!("unexpected number of duplicates at line {line}"));
    }
    Ok(index_to_priority(dup_index))
}

fn solve_second_half(reader: &mut impl BufRead) -> anyhow::Result<u32> {
    let mut total_score = 0;
    let mut lines = reader.lines();
    loop {
        let line = match lines.next() {
            Some(maybe_line) => maybe_line,
            None => break,
        }?;
        let mut badge_index = encode(line.as_str())?;
        badge_index &= next_line(&mut lines)?;
        badge_index &= next_line(&mut lines)?;
        if badge_index.count_ones() != 1 {
            bail!("could not find badge");
        }
        total_score += index_to_priority(badge_index);
    }
    Ok(total_score)
}

fn next_line<B: BufRead>(lines: &mut Lines<B>) -> anyhow::Result<u64> {
    let line = match lines.next() {
        Some(maybe_line) => maybe_line?,
        None => bail!("last group is incomplete"),
    };
    encode(&line)
}

// Transform a sequence of items into a bit mask encoding present items as their
// priority.
fn encode(items: &str) -> anyhow::Result<u64> {
    let mut mask = 0u64;
    for c in items.chars() {
        let bit_index = match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => bail!("unexpected character: {c}"),
        };
        mask |= 1 << bit_index;
    }
    Ok(mask)
}

fn index_to_priority(index: u64) -> u32 {
    index.trailing_zeros() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() -> anyhow::Result<()> {
        assert_eq!(encode("")?, 0);
        assert_eq!(encode("a")?, 1);
        assert_eq!(encode("z")?, (1 << 25));
        assert_eq!(encode("za")?, 1 | (1 << 25));
        assert_eq!(encode("A")?, 1 << 26);
        assert_eq!(encode("Z")?, 1 << 51);
        assert_eq!(encode("ZA")?, (1 << 26) | (1 << 51));
        Ok(())
    }

    #[test]
    fn test_parse_line() -> anyhow::Result<()> {
        assert_eq!(find_duplicated_item("aa")?, 1);
        assert_eq!(find_duplicated_item("AA")?, 27);
        Ok(())
    }

    #[test]
    fn test_index_to_priority() {
        assert_eq!(index_to_priority(1), 1);
        assert_eq!(index_to_priority(2), 2);
        assert_eq!(index_to_priority(4), 3);
    }
}