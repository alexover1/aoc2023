use std::fs;

pub fn run_all() {
    let file_path = "input/1.txt";
    let input = fs::read_to_string(&file_path).expect("failed to read input file");
    println!("Day 01");
    println!("    Part One: {}", part_one(input.as_str()));
    println!("    Part Two: {}", part_two(input.as_str()));
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .map(|digits| as_two_digit_number(&digits))
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| split_string_by_spellings(line))
        .map(|digits| as_two_digit_number(&digits))
        .sum()
}

fn as_two_digit_number(digits: &Vec<u32>) -> u32 {
    let first = digits.iter().next().unwrap();
    let last = digits.iter().rev().next().unwrap();
    first * 10 + last
}

fn split_string_by_spellings(input: & str) -> Vec<u32> {
    let spellings = [
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];

    let mut results = Vec::new();
    let mut temp = String::new();

    for ch in input.chars() {
        temp.push(ch);
        for (num, word) in spellings.iter() {
            if temp.ends_with(num) || temp.ends_with(word) {
                // We only want to add the digit spelling to the list.
                results.push(num.parse::<u32>().unwrap());
                // temp.clear();
            }
        }
    }

    results
}

