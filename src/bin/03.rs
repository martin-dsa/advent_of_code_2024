use fancy_regex::{Captures, Regex};
use lazy_static::lazy_static;

advent_of_code::solution!(3);

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((?P<n1>\d{1,3}),(?P<n2>\d{1,3})\)").unwrap();
    static ref RE_2: Regex =
        Regex::new(r"(?P<dont>don't\(\))|(?P<do>do\(\))|mul\((?P<n1>\d{1,3}),(?P<n2>\d{1,3})\)")
            .unwrap();
}

fn mul(c: &Captures) -> u32 {
    c.name("n1").unwrap().as_str().parse::<u32>().unwrap()
        * c.name("n2").unwrap().as_str().parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = RE
        .captures_iter(input)
        .map(|c| mul(&c.unwrap()))
        .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let res = RE_2
        .captures_iter(input)
        .filter_map(|captures| {
            let captures = captures.unwrap();

            if captures.name("dont").is_some() {
                enabled = false;
                return None;
            }
            if captures.name("do").is_some() {
                enabled = true;
                return None;
            }
            if !enabled {
                return Some(0);
            }
            Some(mul(&captures))
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
