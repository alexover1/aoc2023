use std::fs;
use std::str::FromStr;

pub fn run_all() {
    let file_path = "input/2.txt";
    let contents = fs::read_to_string(&file_path).expect("failed to read input file");
    let input = parse(contents.as_str());
    println!("Day 02");
    println!("    Part One: {}", part_one(&input));
    println!("    Part Two: {}", part_two(&input));
}

pub fn part_one(input: &Vec<Game>) -> u64 {
    let conf = Set::new(12, 13, 14);
    let mut sum = 0;
    'next_game: for game in input {
        for set in &game.sets {
            if !set.is_possible(&conf) {
                continue 'next_game;
            }
        }
        sum += game.id;
    }
    sum
}

pub fn part_two(input: &Vec<Game>) -> u64 {
    input.into_iter().map(|game| game.power()).sum()
}

pub fn parse(input: &str) -> Vec<Game> {
    input.lines().filter_map(|s| s.parse::<Game>().ok()).collect()
}

#[derive(Debug, Default)]
pub struct Set {
    red: u64,
    green: u64,
    blue: u64,
}

impl Set {
    fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }

    /// Returns true if the set is possible with the provided configuration.
    fn is_possible(&self, conf: &Set) -> bool {
        self.red <= conf.red &&
        self.blue <= conf.blue &&
        self.green <= conf.green
    }
}

impl FromStr for Set {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<Vec<&str>> = s
            .split(",")
            .map(|s| s.trim().split_whitespace().collect())
            .collect();

        let mut result: Self = Default::default();

        for item in items {
            if item.len() != 2 {
                return Err(ParseGameError::Format);
            }
            let amount = item[0].parse::<u64>()
                .map_err(|_| ParseGameError::ParseInt)?;
            match item[1] {
                "red"   => result.red = amount,
                "green" => result.green = amount,
                "blue"  => result.blue = amount,
                _ => return Err(ParseGameError::Format),
            }
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub struct Game {
    id: u64,
    sets: Vec<Set>,
} 

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().splitn(2, ": ").collect();
        if parts.len() != 2 {
            return Err(ParseGameError::Format);
        }

        let id = parts[0].trim_start_matches("Game ").parse::<u64>()
            .map_err(|_| ParseGameError::ParseInt)?;

        let sets = parts[1].split("; ").filter_map(|s| s.parse::<Set>().ok()).collect();

        Ok(Game { id, sets })
    }
}

#[derive(Debug)]
pub enum ParseGameError {
    Format,
    ParseInt,
}

impl Game {
    #[allow(dead_code)]
    fn new(id: u64) -> Self {
        Self{
            id,
            sets: Vec::new(),
        }
    }

    fn power(&self) -> u64 {
        let red = self.sets.iter().map(|set| set.red).max().expect("expected at least one set");
        let green = self.sets.iter().map(|set| set.green).max().expect("expected at least one set");
        let blue = self.sets.iter().map(|set| set.blue).max().expect("expected at least one set");
        red * green * blue
    }
}

#[allow(dead_code)]
fn parse2(input: &str) -> Vec<Game> {
    let mut result = Vec::new();

    for line in input.lines().filter_map(|line| non_empty_line(line)) {
        let mut line = line.strip_prefix("Game ").expect("invalid beginning of line");

        let id = {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            line = parts.get(1).unwrap_or(&"");
            parts.get(0).unwrap_or(&"0").parse::<u64>().expect("failed to parse integer id")
        };

        let mut game = Game::new(id);
        for item in line.split(";") {
            let mut bag = Set::default();

            for cube in item.split(",") {
                let parts: Vec<&str> = cube.trim().split_whitespace().collect();
                let amount = parts.get(0).unwrap_or(&"").parse::<u64>().expect("failed to parse integer");
                let color = parts.get(1).unwrap_or(&"");

                match *color {
                    "red" => {
                        bag.red = amount;
                    },
                    "blue" => {
                        bag.blue = amount;
                    },
                    "green" => {
                        bag.green = amount;
                    },
                    _ => {
                        println!("Warning: Invalid bag with color '{color}'");
                    },
                }
            }
            game.sets.push(bag);
        }
        result.push(game);
    }

    result
}

#[allow(dead_code)]
fn non_empty_line<'a>(line: &'a str) -> Option<&'a str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
