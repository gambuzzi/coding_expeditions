use std::collections::HashMap;

fn parser(s: &str) -> HashMap<i64, (Vec<i64>, Vec<i64>)> {
    let mut cards: HashMap<i64, (Vec<i64>, Vec<i64>)> = HashMap::new();

    for line in s.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let card = parts[0];
        let numbers = parts[1];

        let card_parts: Vec<&str> = card.split_whitespace().collect();
        let card_n = card_parts[1].parse::<i64>().unwrap();

        let mut nums = numbers.split(" | ");
        let card_numbers: Vec<i64> = nums
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .filter_map(|x| {
                if let Ok(ret) = x.parse() {
                    Some(ret)
                } else {
                    None
                }
            })
            .collect();
        let win_numbers: Vec<i64> = nums
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .filter_map(|x| {
                if let Ok(ret) = x.parse() {
                    Some(ret)
                } else {
                    None
                }
            })
            .collect();

        cards.insert(card_n, (card_numbers, win_numbers));
    }

    cards
}

pub fn part1(s: &str) -> i64 {
    let data = parser(s);
    let mut ret = 0;

    for (_, (numbers, win_numbers)) in data.iter() {
        let w = numbers.iter().filter(|&x| win_numbers.contains(x)).count();
        if w > 0 {
            ret += 2_i64.pow((w - 1) as u32);
        }
    }

    ret
}

pub fn part2(s: &str) -> i64 {
    let data = parser(s);
    let mut mult: HashMap<i64, i64> = HashMap::new();

    let mut sorted_keys: Vec<i64> = data.keys().cloned().collect();
    sorted_keys.sort();

    for &card_n in &sorted_keys {
        let (numbers, win_numbers) = &data[&card_n];
        mult.entry(card_n).or_insert(1);

        let w = numbers.iter().filter(|&x| win_numbers.contains(x)).count();
        for i in 0..w {
            let next_card = card_n + i as i64 + 1;
            if mult.contains_key(&next_card) {
                let v = mult[&next_card];
                mult.insert(next_card, v + mult[&card_n]);
            } else {
                mult.insert(next_card, mult[&card_n] + 1);
            }
        }
    }

    mult.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let data = parser(s);
        assert_eq!(data.len(), 1);
        assert_eq!(
            data[&1],
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        );
    }
}
