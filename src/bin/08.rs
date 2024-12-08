use std::collections::HashMap;

use advent_of_code::are_collinear;
use itertools::Itertools;

advent_of_code::solution!(8);

fn get_antennas_positions(chars: impl Iterator<Item = char>, size: usize) -> Vec<Vec<(i32, i32)>> {
    chars
        .enumerate()
        .fold(
            HashMap::new(),
            |mut map: HashMap<char, Vec<(i32, i32)>>, (index, c)| {
                if c == '.' {
                    return map;
                }
                map.entry(c)
                    .or_default()
                    .push(((index / size) as i32, (index % size) as i32));
                map
            },
        )
        .into_values()
        .collect()
}

fn get_all_possible_pairs(v: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    v.iter()
        .cartesian_product(v)
        .filter(|(x, y)| **x != **y)
        .map(|(x, y)| (*x, *y))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = input.lines().count();
    let chars = input.chars().filter(|c| *c != '\n' && *c != '\r');
    let antennas = get_antennas_positions(chars, size);

    let antennas_count = antennas
        .iter()
        .flat_map(|same_freq| {
            get_all_possible_pairs(same_freq)
                .into_iter()
                .filter_map(|(p1, p2)| {
                    let vec1 = (p1.0 - p2.0, p1.1 - p2.1);
                    let vec2 = (p2.0 - vec1.0, p2.1 - vec1.1);
                    if vec2.0 < 0 || vec2.0 >= size as i32 || vec2.1 < 0 || vec2.1 >= size as i32 {
                        None
                    } else {
                        Some(vec2)
                    }
                })
        })
        .unique()
        .count();

    Some(antennas_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let size = input.lines().count();
    let chars = input.chars().filter(|c| *c != '\n' && *c != '\r');
    let antennas = get_antennas_positions(chars.clone(), size);

    let antennas_count = chars
        .enumerate()
        .map(|(index, _)| ((index / size) as i32, (index % size) as i32))
        .filter(|index| {
            antennas.iter().any(|same_freq| {
                get_all_possible_pairs(same_freq)
                    .into_iter()
                    .filter(|p| are_collinear(&p.0, &p.1, index))
                    .count()
                    >= 2
            })
        })
        .unique()
        .count();

    Some(antennas_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
