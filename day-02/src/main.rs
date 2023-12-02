use std::fs;
use std::env;
use std::str::FromStr;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Error: This program requires exactly one command-line argument, which should be an input file path.");
        return ExitCode::FAILURE;
    }

    let file_path = &args[0];

    if !Path::new(file_path).exists() {
        eprintln!("Error: The provided file path does not exist.");
        return ExitCode::FAILURE;
    }

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let input = parse(contents.as_str());
            println!("Part One: {}", part_one(&input));
            println!("Part Two: {}", part_two(&input));
            ExitCode::SUCCESS
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            ExitCode::FAILURE
        },
    }
}

fn part_one(input: &Vec<Game>) -> u64 {
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

fn part_two(input: &Vec<Game>) -> u64 {
    input.into_iter().map(|game| game.power()).sum()
}

#[derive(Debug, Default)]
struct Set {
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
struct Game {
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
enum ParseGameError {
    Format,
    ParseInt,
}

impl Game {
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

fn parse(input: &str) -> Vec<Game> {
    input.lines().filter_map(|s| s.parse::<Game>().ok()).collect()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test_part_one() {
        const EXPECTED_OUTPUT: u64 = 8;
        let input = super::parse(TEST_INPUT);
        assert_eq!(
            super::part_one(&input),
            EXPECTED_OUTPUT,
        );
    }

    #[test]
    fn test_part_two() {
        const EXPECTED_OUTPUT: u64 = 2286;
        let input = super::parse(TEST_INPUT);
        assert_eq!(
            super::part_two(&input),
            EXPECTED_OUTPUT,
        );
    }
}

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

fn non_empty_line<'a>(line: &'a str) -> Option<&'a str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

