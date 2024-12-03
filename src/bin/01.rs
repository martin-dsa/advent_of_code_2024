use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    for (l, r) in input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()))
    {
        left_list.push(l);
        right_list.push(r);
    }
    (left_list, right_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = get_lists(input);
    left_list.sort();
    right_list.sort();
    let a = zip(left_list, right_list)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();
    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = get_lists(input);
    let mut mg: HashMap<u32, u32> = HashMap::new();
    for x in right_list {
        let entry = mg.entry(x).or_insert(0);
        *entry += 1;
    }

    let similarity_score = left_list
        .iter()
        .map(|n| n * mg.get(n).unwrap_or(&0))
        .sum::<u32>();
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
