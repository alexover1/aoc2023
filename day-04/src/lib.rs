use std::fs;

pub fn run_all() {
    let file_path = "input/4.txt";
    let input = fs::read_to_string(&file_path).expect("failed to read input file");
    let cards = parse_cards(input.as_str());
    println!("Day 04");
    println!("    Part One: {}", part_one(&cards));
    println!("    Part Two: {}", part_two(&cards));
}

pub fn part_one(cards: &Vec<usize>) -> u64 {
    cards 
       .iter()
       .filter(|x| **x > 0)
       .map(|x| 1 << (x - 1))
       .sum()
}

pub fn part_two(cards: &Vec<usize>) -> usize {
    fn recursively_add_duplicates(dups: &mut Vec<usize>, cards: &Vec<usize>, cur: usize) {
        for i in cur + 1..=cur + cards[cur] {
            recursively_add_duplicates(dups, cards, i);
            dups.push(i);
        }
    }

    let mut dups = Vec::new();
    for i in 0..cards.len() {
        recursively_add_duplicates(&mut dups, &cards, i);
    }

    cards.len() + dups.len()
}

pub fn parse_cards(input: &str) -> Vec<usize> {
    let mut result = Vec::new();

    for line in input.lines() {
        let line = line.strip_prefix("Card").expect("expected line to start with 'Card'");

        let parts: Vec<&str> = line.split(':').map(str::trim).collect();
        assert_eq!(2, parts.len(), "expected exactly two parts");

        // let card_id = parts[0].parse::<u64>().expect("failed to parse integer");

        let card_numbers: Vec<Vec<u64>> = parts[1]
            .split(" | ")
            .map(|part| part.split_whitespace().filter_map(|x| x.parse::<u64>().ok()).collect())
            .collect();
        assert_eq!(2, card_numbers.len(), "expected exactly two parts");

        let n = card_numbers[1]
            .iter()
            .filter(|n| card_numbers[0].contains(n))
            .collect::<Vec<_>>()
            .len();
        result.push(n);
    }

    result
}

