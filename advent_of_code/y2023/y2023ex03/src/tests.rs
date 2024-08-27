#[cfg(test)]
mod tests {
    use y2023ex03::{part1, part2};

    const INPUT: &str = include_str!("../input.txt");
    const README_INPUT: &str = include_str!("../readme_input.txt");

    #[test]
    fn test_part1_custom() {
        let lines: Vec<String> = "123.\n...*".lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&lines), 123);
    }

    #[test]
    fn test_part1_readme() {
        let lines: Vec<String> = README_INPUT.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&lines), 4361);
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();
        assert_eq!(part1(&lines), 553079);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();
        assert_eq!(part2(&lines), 84363105);
    }
}
