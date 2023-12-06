use std::fs;
use std::ops::Range;
use std::str::FromStr;
use regex::Regex;
use itertools::Itertools;

pub fn run_all() {
    let file_path = "input/5.txt";
    let contents = fs::read_to_string(&file_path).expect("failed to read input file");
    
    let tables = parse_transition_tables(&contents).expect("failed to parse transition tables");
    println!("Day 05");
    println!("    Part One: {}", part_one(&contents, &tables));
    println!("    Part Two: {}", part_two(&contents, &tables));
}

pub fn part_one(input: &str, tables: &Vec<Table>) -> u64 {
    let seeds = parse_seeds_part_one(
        input.lines().next().expect("expected first line"),
    ).expect("failed to parse seeds");

    seeds
        .iter()
        .map(|seed| tables.iter().fold(*seed, |acc, table| table.forward(acc)))
        .min()
        .expect("expected at least one seed")
}

pub fn part_two(input: &str, tables: &Vec<Table>) -> u64 {
    let seed_ranges = parse_seeds_part_two(
        input.lines().next().expect("expected first line"),
    ).expect("failed to parse seeds");

    let rev: Vec<Vec<RangePair>> = tables
        .iter()
        .rev()
        .map(|table| table.matches.iter().map(RangePair::flip).collect())
        .collect();

    (0..)
        .find(|&loc| {
            let seed = rev.iter().fold(loc, |acc, ranges| {
                ranges
                    .iter()
                    .find(|range| range.src.contains(&acc))
                    .map_or(acc, |range| range.translate(acc))
            });
            seed_ranges.iter().any(|seed_range| seed_range.contains(&seed))
        })
        .unwrap()
}

/// A range pair is a mapping from a source range
/// to a destination range.
pub struct RangePair {
    src: Range<u64>,
    dst: Range<u64>,
}

impl RangePair {
    fn flip(&self) -> Self {
        Self { src: self.dst.clone(), dst: self.src.clone() }
    }

    fn contains(&self, num: u64) -> bool {
        self.src.contains(&num)
    }

    fn translate(&self, num: u64) -> u64 {
        self.dst.start + num - self.src.start
    }
}

impl FromStr for RangePair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().map(|x| x.parse().map_err(|_| ParseError::ParseInt));

        let dst = parts.next().ok_or(ParseError::Format("Expected `dst` field".into()))??;
        let src = parts.next().ok_or(ParseError::Format("Expected `src` field".into()))??;
        let len = parts.next().ok_or(ParseError::Format("Expected `len` field".into()))??;
        assert_eq!(None, parts.next());

        Ok(Self {
            src: src..src + len,
            dst: dst..dst + len,
        })
    }
}

/// A table is a list of transitions from source to
/// destination ranges.
///
/// For example:
///     `[0..10] => [50..60]`
///     `[20..25] => [80..85]`
///     `_ => x`
pub struct Table {
    matches: Vec<RangePair>,
}

impl Table {
    /// Maps the number to the corresponding range from
    /// the list of matches in the transition table, or
    /// returns the original number.
    fn forward(&self, x: u64) -> u64 {
        self.matches
            .iter()
            .find(|&range_pair| range_pair.contains(x))
            .map(|range_pair| range_pair.translate(x))
            .unwrap_or(x)
        
        // self.matches.iter().fold(x, |acc, range_pair| range_pair.translate(acc))
    }
}

impl FromStr for Table {
    type Err = ParseError;

    /// Note: The input is expected to include a line of the format
    /// `seed-to-soil map:`, but the implementation currently just
    /// ignores the first line.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = vec![];
        for line in s.lines().skip(1) {
            matches.push(line.parse()?);
        }
        Ok(Self { matches })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Format(String),
    ParseInt,
    Regex,
}

/// Parses a list of numbers corresponding to a list of seeds.
pub fn parse_seeds_part_one(first_line: &str) -> Result<Vec<u64>, ParseError> {
    let result = first_line 
        .strip_prefix("seeds: ")
        .ok_or(ParseError::Format("Invalid first line".into()))?
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    Ok(result)
}

/// Parses a list of pairs of numbers corresponding to `pairs` of seeds,
/// where the first number is the starting seed, and the second is the
/// length of the range: `start..start + length`.
pub fn parse_seeds_part_two(first_line: &str) -> Result<Vec<Range<u64>>, ParseError> {
    let result = first_line
        .strip_prefix("seeds: ")
        .ok_or(ParseError::Format("Invalid first line".into()))?
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .tuples()
        .map(|(start, length)| start..start + length)
        .collect();
    Ok(result)
}

/// Note: The input is expected to not include the first line,
/// which starts with `seeds:`.
pub fn parse_transition_tables(input: &str) -> Result<Vec<Table>, ParseError> {
    let re = Regex::new(r"\n\s*\n").map_err(|_| ParseError::Regex)?;
    let mut result = Vec::new();
    for region in re.split(input) {
        result.push(region.parse()?);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two, parse_transition_tables, Table};

    const INPUT: &str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    #[test]
    fn test_part_one() {
        let tables = parse_transition_tables(INPUT).expect("failed to parse transition tables");
        assert_eq!(35, part_one(INPUT, &tables));
    }

    #[test]
    fn test_part_two() {
        let tables = parse_transition_tables(INPUT).expect("failed to parse transition tables");
        assert_eq!(46, part_two(INPUT, &tables));
    }

    #[test]
    fn test_parse_tables() {
        let input = concat!(
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
        );

        let table = input.parse::<Table>().expect("failed to parse table");
        assert_eq!(2, table.matches.len());
        assert_eq!(50, table.matches[0].dst.start);
        assert_eq!(98, table.matches[0].src.start);
    }
}
