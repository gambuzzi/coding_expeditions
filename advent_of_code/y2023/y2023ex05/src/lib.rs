use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Mappings {
    destination: i64,
    source: i64,
    length: i64,
}

fn parser(s: &str) -> (Vec<i64>, HashMap<String, Vec<Mappings>>) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut a_to_b: Option<String> = None;
    let mut a_to_b_map: HashMap<String, Vec<Mappings>> = HashMap::new();

    for line in s.lines() {
        if line.starts_with("seeds:") {
            seeds = line
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect();
        } else if line.contains("map:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            a_to_b = parts.get(0).map(|&x| x.to_string());
        } else if let Some(key) = &a_to_b {
            if !line.is_empty() {
                let values: Vec<i64> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let destination = values[0];
                let source = values[1];
                let length = values[2];

                let mappings = Mappings {
                    destination,
                    source,
                    length,
                };

                let entry = a_to_b_map.entry(key.clone()).or_insert_with(Vec::new);
                entry.push(mappings);
            }
        }
    }

    (seeds, a_to_b_map)
}

fn lookup_part1(k: &str, n: i64, a_to_b: &HashMap<String, Vec<Mappings>>) -> i64 {
    if let Some(mapping) = a_to_b.get(k) {
        for m in mapping {
            if n >= m.source && n < m.source + m.length {
                return m.destination + (n - m.source);
            }
        }
    }
    n
}

pub fn part1(s: &str) -> i64 {
    let (seeds, a_to_b) = parser(s);
    let mut min = (u64::MAX >> 1) as i64;

    for seed in seeds {
        let mut n = seed;
        n = lookup_part1("seed-to-soil", n, &a_to_b);
        n = lookup_part1("soil-to-fertilizer", n, &a_to_b);
        n = lookup_part1("fertilizer-to-water", n, &a_to_b);
        n = lookup_part1("water-to-light", n, &a_to_b);
        n = lookup_part1("light-to-temperature", n, &a_to_b);
        n = lookup_part1("temperature-to-humidity", n, &a_to_b);
        n = lookup_part1("humidity-to-location", n, &a_to_b);
        if n < min {
            min = n
        }
    }

    min
}

#[derive(Debug)]
struct NumberRange {
    range: Vec<(i64, i64)>,
}

impl NumberRange {
    fn new() -> Self {
        NumberRange { range: Vec::new() }
    }

    fn remove_range(&mut self, start: i64, end: i64) {
        let mut new_ranges = Vec::new();
        for (r_start, r_end) in &self.range {
            if end < *r_start || start > *r_end {
                new_ranges.push((*r_start, *r_end));
            } else if start <= *r_start && end >= *r_end {
                // entirely covers the current range
                // do nothing
            } else {
                if start <= *r_start {
                    new_ranges.push((end + 1, *r_end));
                } else if end >= *r_end {
                    new_ranges.push((*r_start, start - 1));
                } else {
                    new_ranges.push((*r_start, start - 1));
                    new_ranges.push((end + 1, *r_end));
                }
            }
        }
        self.range = new_ranges;
    }

    fn compute_overlap(&self, other_start: i64, other_end: i64) -> Vec<(i64, i64)> {
        let mut overlaps = Vec::new();
        for (r_start, r_end) in &self.range {
            if other_end < *r_start || other_start > *r_end {
                continue;
            } else {
                let overlap_start = other_start.max(*r_start);
                let overlap_end = other_end.min(*r_end);
                overlaps.push((overlap_start, overlap_end));
            }
        }
        overlaps
    }
}

fn lookup_part2(
    k: &str,
    range: &mut NumberRange,
    a_to_b: &HashMap<String, Vec<Mappings>>,
) -> NumberRange {
    let mut ret = NumberRange::new();

    if let Some(mappings) = a_to_b.get(k) {
        for mapping in mappings {
            let (destination, source, length) =
                (mapping.destination, mapping.source, mapping.length);
            let mapped_range = range.compute_overlap(source, source + length - 1);
            if !mapped_range.is_empty() {
                for m in &mapped_range {
                    range.remove_range(m.0, m.1);
                }

                let mut mapped_range_new = Vec::new();
                for m in mapped_range {
                    let mapped = (destination + (m.0 - source), destination + (m.1 - source));
                    mapped_range_new.push(mapped);
                }
                ret.range.extend(mapped_range_new);
            }
        }
    }

    if !range.range.is_empty() {
        ret.range.extend(range.range.clone());
    }

    ret
}

pub fn part2(s: &str) -> i64 {
    let (seeds, a_to_b) = parser(s);
    let mut n = NumberRange::new();

    for chunk in seeds.chunks(2) {
        if let Some(seed_start) = chunk.get(0) {
            if let Some(seed_width) = chunk.get(1) {
                n.range.push((*seed_start, *seed_start + *seed_width - 1));
            }
        }
    }

    let keys = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    for key in keys {
        n = lookup_part2(key, &mut n, &a_to_b);
    }

    let min_val = n.range.iter().map(|x| x.0).min();

    min_val.unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const README_INPUT: &str = include_str!("../readme_input.txt");

    #[test]
    fn test_parser() {
        let (seeds, mappings) = parser(README_INPUT);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        let mut expected_mappings = HashMap::new();

        expected_mappings.insert(
            "light-to-temperature".to_string(),
            vec![
                Mappings {
                    destination: 45,
                    source: 77,
                    length: 23,
                },
                Mappings {
                    destination: 81,
                    source: 45,
                    length: 19,
                },
                Mappings {
                    destination: 68,
                    source: 64,
                    length: 13,
                },
            ],
        );
        expected_mappings.insert(
            "seed-to-soil".to_string(),
            vec![
                Mappings {
                    destination: 50,
                    source: 98,
                    length: 2,
                },
                Mappings {
                    destination: 52,
                    source: 50,
                    length: 48,
                },
            ],
        );
        expected_mappings.insert(
            "water-to-light".to_string(),
            vec![
                Mappings {
                    destination: 88,
                    source: 18,
                    length: 7,
                },
                Mappings {
                    destination: 18,
                    source: 25,
                    length: 70,
                },
            ],
        );
        expected_mappings.insert(
            "fertilizer-to-water".to_string(),
            vec![
                Mappings {
                    destination: 49,
                    source: 53,
                    length: 8,
                },
                Mappings {
                    destination: 0,
                    source: 11,
                    length: 42,
                },
                Mappings {
                    destination: 42,
                    source: 0,
                    length: 7,
                },
                Mappings {
                    destination: 57,
                    source: 7,
                    length: 4,
                },
            ],
        );
        expected_mappings.insert(
            "humidity-to-location".to_string(),
            vec![
                Mappings {
                    destination: 60,
                    source: 56,
                    length: 37,
                },
                Mappings {
                    destination: 56,
                    source: 93,
                    length: 4,
                },
            ],
        );
        expected_mappings.insert(
            "temperature-to-humidity".to_string(),
            vec![
                Mappings {
                    destination: 0,
                    source: 69,
                    length: 1,
                },
                Mappings {
                    destination: 1,
                    source: 0,
                    length: 69,
                },
            ],
        );
        expected_mappings.insert(
            "soil-to-fertilizer".to_string(),
            vec![
                Mappings {
                    destination: 0,
                    source: 15,
                    length: 37,
                },
                Mappings {
                    destination: 37,
                    source: 52,
                    length: 2,
                },
                Mappings {
                    destination: 39,
                    source: 0,
                    length: 15,
                },
            ],
        );

        for key in mappings.keys() {
            assert_eq!(mappings[key], expected_mappings[key], "{}", key);
        }
    }
}
