use std::collections::HashMap;

advent_of_code::solution!(11);

fn map_stone(stone: u128, n: usize, cache: &mut HashMap<(u128, usize), u128>) -> u128 {
    if n == 0 {
        return 1;
    }
    let key = (stone, n);
    if let Some(val) = cache.get(&key) {
        return *val;
    }
    let val = match stone {
        0 => map_stone(1, n - 1, cache),
        _ => {
            let str = stone.to_string();
            if str.len() % 2 == 0 {
                let a = str.split_at(str.len() / 2);
                map_stone(a.0.parse::<u128>().unwrap(), n - 1, cache)
                    + map_stone(a.1.parse::<u128>().unwrap(), n - 1, cache)
            } else {
                map_stone(stone * 2024, n - 1, cache)
            }
        }
    };
    cache.insert(key, val);
    val
}

fn solve(input: &str, n: usize) -> Option<u128> {
    let mut cache: HashMap<(u128, usize), u128> = HashMap::new();
    Some(
        input
            .split_whitespace()
            .map(|s| s.parse::<u128>().unwrap())
            .map(|s| map_stone(s, n, &mut cache))
            .sum::<u128>(),
    )
}

pub fn part_one(input: &str) -> Option<u128> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u128> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
