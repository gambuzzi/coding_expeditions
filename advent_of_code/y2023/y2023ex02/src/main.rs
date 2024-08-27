use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parser(s: &Vec<String>) -> HashMap<String, Vec<Vec<Vec<&str>>>> {
    let mut ret: HashMap<String, Vec<Vec<Vec<&str>>>> = HashMap::new();

    for line in s.iter() {
        let mut iter = line.split(':');
        if let (Some(game), Some(raw_rounds)) = (iter.next(), iter.next()) {
            let tmp = game.split_whitespace().collect::<Vec<&str>>();
            let (_, game_id) = tmp.split_at(1);
            let game = game_id.join(" ");

            let rounds: Vec<&str> = raw_rounds.trim().split("; ").collect();
            let mut rounds_list: Vec<Vec<Vec<&str>>> = Vec::new();

            for r in rounds {
                let cubes: Vec<Vec<&str>> = r
                    .split(", ")
                    .map(|x| x.split_whitespace().collect())
                    .collect();
                rounds_list.push(cubes);
            }

            ret.insert(game, rounds_list);
        }
    }

    ret
}

fn check_limits(games: &HashMap<String, Vec<Vec<Vec<&str>>>>) -> i32 {
    let mut ret = 0;
    let limits: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    for (game_id, rounds) in games.iter() {
        let mut valid_game = true;

        for round in rounds {
            for cube in round {
                if cube.len() == 2 {
                    let count = cube[0];
                    let cube_type = cube[1];

                    if let Some(&limit) = limits.get(cube_type) {
                        if count.parse::<i32>().unwrap_or(0) > limit {
                            valid_game = false;
                            break;
                        }
                    }
                } else {
                    valid_game = false;
                    break;
                }
            }
            if !valid_game {
                break;
            }
        }

        if valid_game {
            ret += game_id.parse::<i32>().unwrap_or(0);
        }
    }

    ret
}

fn part2(games: &HashMap<String, Vec<Vec<Vec<&str>>>>) -> i32 {
    let mut ret = 0;

    for (_, rounds) in games.iter() {
        let mut least: HashMap<&str, i32> = [("red", 0), ("green", 0), ("blue", 0)]
            .into_iter()
            .collect();

        for r in rounds {
            for cube in r {
                if let [count, color] = cube.as_slice() {
                    let count = count.parse::<i32>().unwrap_or(0);
                    if let Some(value) = least.get_mut::<&str>(color) {
                        *value = (*value).max(count);
                    }
                }
            }
        }

        let power: i32 = least.values().product();
        ret += power;
    }

    ret
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let games = parser(&lines);

    let part1: i32 = check_limits(&games);
    println!("part1 {}", part1);

    let part2: i32 = part2(&games);
    println!("part2 {}", part2);

    Ok(())
}
