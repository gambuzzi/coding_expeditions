use std::cmp::min;

fn find(line: &str, ptr: usize, only_digits: bool) -> Option<i32> {
    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let max_ptr = line.len();
    for (k, v) in digits.iter() {
        if only_digits && k.len() > 1 {
            continue;
        }
        if line[ptr..min(max_ptr, ptr + k.len())] == **k {
            return Some(*v);
        }
    }
    None
}

pub fn solve(lines: &Vec<String>, only_digits: bool) -> i32 {
    lines
        .iter()
        .map(|line| {
            let left = (0..line.len())
                .find_map(|i| find(&line, i, only_digits))
                .unwrap();
            let right = (0..line.len())
                .rev()
                .find_map(|i| find(&line, i, only_digits))
                .unwrap();
            left * 10 + right
        })
        .sum()
}
