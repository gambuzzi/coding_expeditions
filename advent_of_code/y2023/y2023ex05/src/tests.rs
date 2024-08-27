#[cfg(test)]
mod tests {
    use y2023ex05::{part1, part2};
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 806029445);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 59370572);
    }
}
