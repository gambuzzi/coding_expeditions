use y2023ex01::solve;

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        let lines: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();
        assert_eq!(solve(&lines, true), 53386);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();
        assert_eq!(solve(&lines, false), 53312);
    }
}
