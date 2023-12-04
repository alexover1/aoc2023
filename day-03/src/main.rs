fn main() {
    day_03::run_all();
}

#[cfg(test)]
mod tests {
    use day_03::{Graph, part_one, part_two};

    const INPUT: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598..\n",
    );

    #[test]
    fn test_part_one() {
        let graph = INPUT.parse::<Graph>().expect("failed to parse input");
        assert_eq!(4361, part_one(&graph));
    }

    #[test]
    fn test_part_two() {
        let graph = INPUT.parse::<Graph>().expect("failed to parse input");
        assert_eq!(467835, part_two(&graph));
    }
}
