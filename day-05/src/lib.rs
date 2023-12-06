use std::fs;
use std::ops::Range;
use std::str::FromStr;
use regex::Regex;

use std::thread;
use std::sync::Arc;

pub fn run_all() {
    let file_path = "input/5.txt";
    let contents = fs::read_to_string(&file_path).expect("failed to read input file");
    
    let almanac = contents.parse::<Almanac>().expect("failed to parse input");
    println!("Day 05");
    println!("    Part One: {}", part_one(&almanac));
    println!("    Part Two: {}", part_two_ext(&almanac));
}

pub fn part_one(almanac: &Almanac) -> u64 {
    almanac.seeds
        .iter()
        .map(|seed| almanac.maps.iter().fold(*seed, |acc, x| x.translate(acc)))
        .min()
        .expect("expected at least one seed")
}

pub fn part_two_ext(input: &Almanac) -> u64 {
    /*
    let mut ranges = vec![];

    // Parse a list of ranges from the input.

    for line in input.lines().filter(|line| line.starts_with(char::is_ascii_digit)) {
        let parts = line.split_whitespace().filter_map(|x| x.parse().ok());

        let dst = parts.next().unwrap();
        let src = parts.next().unwrap();
        let len = parts.next().unwrap();
        assert_eq!(None, parts.next());

        ranges.push(Range::new(dst, src, len));
    }
    */

    let all_seeds: Vec<Range<u64>> = input.seeds
        .chunks(2)
        .filter_map(|slice| match slice {
            &[start, length] => Some(start..start + length),
            _ => None,
        })
        .collect();

    let rev: Vec<Vec<RangePair>> = input.maps
        .iter()
        .rev()
        .map(|map| map.ranges.iter().map(RangePair::flip).collect())
        .collect();

    (0..)
        .find(|&loc| {
            let seed = rev.iter().fold(loc, |acc, ranges| {
                ranges
                    .iter()
                    .find(|range| range.src.contains(&acc))
                    .map_or(acc, |range| range.translate(acc))
            });
            all_seeds.iter().any(|seed_range| seed_range.contains(&seed))
        })
        .unwrap()
}

pub fn part_two(almanac: Almanac) -> u64 {
    let pairs: Vec<(u64, u64)> = almanac.seeds
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None // Ignore incomplete pairs
            }
        })
        .collect();
    
    let data = Arc::new(almanac);
    let mut handles = vec![];

    for (start, length) in pairs {
        let almanac = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut low = u64::MAX;
            for seed in start..start + length {
                let cur = almanac.process(seed);
                if cur < low {
                    low = cur;
                }
            }
            low
        });
        handles.push(handle);
    }

    let mut low = u64::MAX;
    for handle in handles {
        let cur = handle.join().unwrap();
        if cur < low {
            low = cur;
        }
    }
    low
}

#[derive(Debug)]
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().map(|x| x.parse().unwrap());

        let dst = parts.next().unwrap();
        let src = parts.next().unwrap();
        let len = parts.next().unwrap();
        assert_eq!(None, parts.next());

        Ok(Self {
            src: src..src + len,
            dst: dst..dst + len,
        })
    }
}

#[derive(Debug)]
pub struct Map {
    // name: String,
    ranges: Vec<RangePair>,
}

impl Map {
    fn translate(&self, num: u64) -> u64 {
        for pair in &self.ranges {
            if pair.contains(num) {
                return pair.translate(num);
            }
        }
        return num;
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().expect("expected at least one line");
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() {
            ranges.push(line.parse().unwrap());
        }
        Ok(Self { ranges })
    }
}

#[derive(Debug)]
pub enum ParseError {
    Format,
    ParseInt,
    Regex,
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn process(&self, seed: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed, |acc, map| map.translate(acc))
    }

    /*fn build_hash_map(&self) -> HashMap<u64, u64> {
        for map in &self.maps {
            for range in map.ranges {
            }
        }
    }*/
}

impl FromStr for Almanac {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\n\s*\n").map_err(|_| ParseError::Regex)?;
        let mut regions = re.split(s);
        let first_line = regions.next().ok_or(ParseError::Format)?;
        let seeds: Vec<u64> = first_line
            .strip_prefix("seeds:")
            .ok_or(ParseError::Format)?
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        let mut maps = Vec::new();
        while let Some(region) = regions.next() {
            maps.push(region.parse()?);
        }
        Ok(Self { seeds, maps })
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, Almanac, Map};

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
        let almanac = INPUT.parse::<Almanac>().expect("failed to parse input");
        assert_eq!(35, part_one(&almanac));
    }

    #[test]
    fn test_parse_almanac() {
        let almanac = INPUT.parse::<Almanac>().expect("failed to parse almanac");
        assert_eq!(4, almanac.seeds.len());
        assert_eq!(7, almanac.maps.len());
    }

    #[test]
    fn test_parse_map() {
        let input = concat!(
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
        );

        let map = input.parse::<Map>().expect("failed to parse map");
        assert_eq!(2, map.ranges.len());
        assert_eq!(50, map.ranges[0].destination_start);
        assert_eq!(98, map.ranges[0].source_start);
        assert_eq!(2, map.ranges[0].range_length);

        println!("First:");
        assert_eq!(50, map.next(98));
        println!("Second:");
        assert_eq!(51, map.next(99));
    }
}
