use std::io::BufRead;

use anyhow::bail;
use util::{self, Half, Parser};

fn main() -> anyhow::Result<()> {
    util::driver(3, DayThreeParser {})
}

struct DayThreeParser;

impl Parser for DayThreeParser {
    fn parse(&mut self, half: Half, reader: &mut impl BufRead) -> anyhow::Result<u32> {
        let mut total_score = 0u32;
        for maybe_line in reader.lines() {
            let line = maybe_line?;
            total_score += parse_line(&line, half)?;
        }
        Ok(total_score)
    }
}

fn parse_line(line: &str, _half: Half) -> anyhow::Result<u32> {
    if line.len() % 2 != 0 {
        bail!(format!("odd number of characters: {line}"))
    }
    let midpoint = line.len() / 2;
    let lhs = encode(&line[0..midpoint])?;
    let rhs = encode(&line[midpoint..])?;
    let dup_index = lhs & rhs;
    if dup_index.count_ones() != 1 {
        bail!("unexpected number of duplicates at line {line}");
    }
    Ok(dup_index.trailing_zeros() + 1)
}

fn encode(s: &str) -> anyhow::Result<u64> {
    let mut mask = 0u64;
    for c in s.chars() {
        let bit_index = match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => bail!("unexpected character: {c}"),
        };
        mask |= 1 << bit_index;
    }
    Ok(mask)
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
        assert_eq!(parse_line("aa", Half::First)?, 1);
        assert_eq!(parse_line("AA", Half::First)?, 27);
        Ok(())
    }
}
