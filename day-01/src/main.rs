fn main() {
    let input = include_str!("./input1.txt");
    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .map(|digits| as_two_digit_number(&digits))
        .sum()
}

fn part_two(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = include_str!("./test1.txt");
        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("./test2.txt");
        assert_eq!(part_two(input), 281);
    }
}
