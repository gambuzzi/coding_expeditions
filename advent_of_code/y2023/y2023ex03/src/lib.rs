use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
struct Number {
    number: i32,
    x: usize,
    y: usize,
    length: usize,
}

#[derive(PartialEq, Debug)]
struct Symbol {
    symbol: String,
    x: usize,
    y: usize,
}

fn parser(s: &Vec<String>) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut number_acc = String::new();
    for (y, row) in s.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell.is_digit(10) {
                number_acc.push(cell);
            } else {
                if !number_acc.is_empty() {
                    numbers.push(Number {
                        number: number_acc.parse().unwrap(),
                        x,
                        y,
                        length: number_acc.len(),
                    });
                    number_acc.clear();
                }
                if cell != '.' {
                    symbols.push(Symbol {
                        symbol: cell.to_string(),
                        x,
                        y,
                    });
                }
            }
        }
        if !number_acc.is_empty() {
            numbers.push(Number {
                number: number_acc.parse().unwrap(),
                x: row.len(),
                y,
                length: number_acc.len(),
            });
            number_acc.clear();
        }
    }
    (numbers, symbols)
}

fn adj_coords(x: usize, y: usize, length: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut coordinates = Vec::new();

    coordinates.push((x.saturating_sub(length + 1), y));
    coordinates.push((x, y));

    for i in 0..=length + 1 {
        coordinates.push((x.saturating_sub(length + 1) + i, y.saturating_sub(1)));
        coordinates.push((x.saturating_sub(length + 1) + i, y + 1));
    }

    coordinates.into_iter()
}

pub fn part1(lines: &Vec<String>) -> i32 {
    let (numbers, symbols) = parser(lines);
    let symbols_values: HashSet<_> = symbols.iter().map(|e| (e.x, e.y)).collect();

    let mut ret = 0;
    for number in numbers {
        if adj_coords(number.x, number.y, number.length).any(|e| symbols_values.contains(&e)) {
            ret += number.number;
        }
    }

    ret
}

pub fn part2(lines: &Vec<String>) -> i32 {
    let (numbers, symbols) = parser(lines);
    let gear: HashSet<_> = symbols
        .iter()
        .filter_map(|e| {
            if e.symbol == "*" {
                Some((e.x, e.y))
            } else {
                None
            }
        })
        .collect();
    let mut ratios: HashMap<(usize, usize), (i32, i32)> = HashMap::new();

    for number in numbers {
        for e in adj_coords(number.x, number.y, number.length) {
            if gear.contains(&e) {
                if let Some(entry) = ratios.get_mut(&e) {
                    if entry.1 == 1 {
                        *entry = (number.number * entry.0, 2);
                    } else {
                        *entry = (0, 3);
                    }
                } else {
                    ratios.insert(e, (number.number, 1));
                }
                break;
            }
        }
    }

    ratios.values().filter(|x| x.1 == 2).map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../readme_input.txt");

    #[test]
    fn test_adj_coords() {
        let coords = adj_coords(4, 4, 2);
        let coords: Vec<_> = coords.collect();
        assert_eq!(
            coords,
            vec![
                (1, 4),
                (4, 4),
                (1, 3),
                (1, 5),
                (2, 3),
                (2, 5),
                (3, 3),
                (3, 5),
                (4, 3),
                (4, 5)
            ]
        );
    }
    #[test]
    fn test_adj_coords_2() {
        let coords = adj_coords(2, 0, 2);
        let coords: Vec<_> = coords.collect();
        assert_eq!(
            coords,
            vec![
                (0, 0),
                (2, 0),
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 0),
                (2, 1),
                (3, 0),
                (3, 1)
            ]
        );
    }
    #[test]
    fn test_parser() {
        let lines: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();

        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        numbers.push(Number {
            number: 467,
            x: 3,
            y: 0,
            length: 3,
        });
        numbers.push(Number {
            number: 114,
            x: 8,
            y: 0,
            length: 3,
        });
        numbers.push(Number {
            number: 35,
            x: 4,
            y: 2,
            length: 2,
        });
        numbers.push(Number {
            number: 633,
            x: 9,
            y: 2,
            length: 3,
        });
        numbers.push(Number {
            number: 617,
            x: 3,
            y: 4,
            length: 3,
        });
        numbers.push(Number {
            number: 58,
            x: 9,
            y: 5,
            length: 2,
        });
        numbers.push(Number {
            number: 592,
            x: 5,
            y: 6,
            length: 3,
        });
        numbers.push(Number {
            number: 755,
            x: 9,
            y: 7,
            length: 3,
        });
        numbers.push(Number {
            number: 664,
            x: 4,
            y: 9,
            length: 3,
        });
        numbers.push(Number {
            number: 598,
            x: 8,
            y: 9,
            length: 3,
        });
        symbols.push(Symbol {
            symbol: "*".to_string(),
            x: 3,
            y: 1,
        });
        symbols.push(Symbol {
            symbol: "#".to_string(),
            x: 6,
            y: 3,
        });
        symbols.push(Symbol {
            symbol: "*".to_string(),
            x: 3,
            y: 4,
        });
        symbols.push(Symbol {
            symbol: "+".to_string(),
            x: 5,
            y: 5,
        });
        symbols.push(Symbol {
            symbol: "$".to_string(),
            x: 3,
            y: 8,
        });
        symbols.push(Symbol {
            symbol: "*".to_string(),
            x: 5,
            y: 8,
        });

        assert_eq!(parser(&lines), (numbers, symbols));
    }
}
