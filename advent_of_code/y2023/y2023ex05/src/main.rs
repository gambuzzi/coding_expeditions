mod tests;

use std::fs::read_to_string;

use y2023ex05::{part1, part2};

fn main() -> Result<(), std::io::Error> {
    let input = read_to_string("./input.txt")?;

    let part1 = part1(input.as_str());
    println!("part1 {}", part1);

    let part2 = part2(input.as_str());
    println!("part2 {}", part2);

    Ok(())
}
