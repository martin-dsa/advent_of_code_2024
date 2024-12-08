use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

fn is_valid(rules_map: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    update.iter().enumerate().all(|(a_pos, a)| {
        rules_map.get(a).map_or(true, |rule| {
            rule.iter().all(|b| {
                update
                    .iter()
                    .position(|n| n == b)
                    .map_or(true, |b_pos| b_pos > a_pos)
            })
        })
    })
}

fn sort_update(rules_map: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    let mut update = update.to_vec();
    update.sort_by(|a, b| match rules_map.get(b).map(|rule| rule.contains(a)) {
        Some(true) => Ordering::Greater,
        _ => Ordering::Less,
    });
    update
}

fn parse_data(input: &str) -> (HashMap<u32, Vec<u32>>, impl Iterator<Item = Vec<u32>> + '_) {
    let (rules, updates) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .unwrap();

    let rules_map = rules
        .lines()
        .fold(HashMap::new(), |mut map: HashMap<u32, Vec<u32>>, line| {
            let (key, value) = line
                .split_once('|')
                .map(|(k, v)| (k.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()))
                .unwrap();
            map.entry(key).or_default().push(value);
            map
        });

    let updates = updates.lines().map(|line| {
        line.split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });

    (rules_map, updates)
}

fn middle_element<T: Clone>(vec: Vec<T>) -> Option<T> {
    if vec.is_empty() {
        None
    } else {
        Some((vec.get(vec.len() / 2).unwrap()).clone())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_map, updates) = parse_data(input);

    let res = updates
        .filter_map(|u| {
            if !is_valid(&rules_map, &u) {
                None
            } else {
                middle_element(u)
            }
        })
        .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_map, updates) = parse_data(input);

    let res = updates
        .filter_map(|u| {
            if is_valid(&rules_map, &u) {
                None
            } else {
                middle_element(sort_update(&rules_map, &u))
            }
        })
        .sum::<u32>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
