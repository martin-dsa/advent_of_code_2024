use std::{convert::Infallible, str::FromStr};

advent_of_code::solution!(7);

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concatenate(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(": ")
            .map(|x| Equation {
                result: x.0.parse::<u64>().unwrap(),
                numbers: x.1.split(' ').map(|n| n.parse::<u64>().unwrap()).collect(),
            })
            .unwrap())
    }
}

impl Equation {
    fn all_permutations(
        mut numbers: Vec<u64>,
        mut result: Vec<u64>,
        operators: Vec<fn(u64, u64) -> u64>,
    ) -> Vec<u64> {
        if numbers.is_empty() {
            return result;
        }

        if result.is_empty() {
            result.push(numbers[0]);
            numbers = numbers[1..].to_vec();
        }

        let new_res = result
            .iter()
            .flat_map(|n| operators.iter().map(|op| op(*n, numbers[0])))
            .collect();

        Equation::all_permutations(numbers[1..].to_vec(), new_res, operators)
    }

    fn is_valid(&self, operators: Vec<fn(u64, u64) -> u64>) -> bool {
        Equation::all_permutations(self.numbers.clone(), vec![], operators)
            .iter()
            .any(|n| *n == self.result)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| l.parse::<Equation>().unwrap())
        .filter_map(|eq| {
            if eq.is_valid(vec![add, multiply]) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| l.parse::<Equation>().unwrap())
        .filter_map(|eq| {
            if eq.is_valid(vec![add, multiply, concatenate]) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum::<u64>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
