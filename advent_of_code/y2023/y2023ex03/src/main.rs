mod tests;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use y2023ex03::{part1, part2};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let part1: i32 = part1(&lines);
    println!("part1 {}", part1);

    let part2: i32 = part2(&lines);
    println!("part2 {}", part2);

    Ok(())
}
