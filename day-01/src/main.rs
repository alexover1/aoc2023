fn main() {
    day_01::run_all();
}

#[cfg(test)]
mod tests {
    use day_01::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = include_str!("test1.txt");
        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test2.txt");
        assert_eq!(part_two(input), 281);
    }
}
