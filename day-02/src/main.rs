fn main() {
    day_02::run_all();
}

#[cfg(test)]
mod tests {
    use day_02::{parse, part_one, part_two};

    const INPUT: &str = concat!(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
    );

    #[test]
    fn test_part_one() {
        let input = parse(INPUT);
        assert_eq!(8, part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = parse(INPUT);
        assert_eq!(2286, part_two(&input));
    }
}
