use advent_of_code::Vec2;
use fancy_regex::Regex;
use lazy_static::lazy_static;

advent_of_code::solution!(13);

lazy_static! {
    static ref BUTTON_REGEX: Regex = Regex::new(r"Button (?:A|B): X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_REGEX: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

fn parse_line(s: &str, re: &Regex) -> Vec2<u128> {
    let captures = re.captures(s).unwrap().unwrap();
    let x = captures.get(1).unwrap().as_str().parse::<u128>().unwrap();
    let y = captures.get(2).unwrap().as_str().parse::<u128>().unwrap();

    Vec2 { x, y }
}

fn parse(
    input: &str,
    prize_m: u128,
) -> impl Iterator<Item = (Vec2<u128>, Vec2<u128>, Vec2<u128>)> + '_ {
    let sections = if input.contains("\n\n") {
        input.split("\n\n")
    } else {
        input.split("\r\n\r\n")
    };

    sections.map(move |s| {
        let mut lines = s.lines();
        let button_a = parse_line(lines.next().unwrap(), &BUTTON_REGEX);
        let button_b = parse_line(lines.next().unwrap(), &BUTTON_REGEX);
        let prize = parse_line(lines.next().unwrap(), &PRIZE_REGEX);
        (
            button_a,
            button_b,
            Vec2 {
                x: prize.x + prize_m,
                y: prize.y + prize_m,
            },
        )
    })
}

fn solve((button_a, button_b, prize): &(Vec2<u128>, Vec2<u128>, Vec2<u128>)) -> Option<Vec2<u128>> {
    let Vec2 { x: a_x, y: a_y } = button_a;
    let Vec2 { x: b_x, y: b_y } = button_b;
    let Vec2 { x: c_x, y: c_y } = prize;

    let det = (a_x * b_y).checked_sub(a_y * b_x);

    if det.is_none() || det.unwrap() == 0 {
        None
    } else {
        let det = det.unwrap();

        if (c_x * b_y - c_y * b_x) % det != 0 || (a_x * c_y - a_y * c_x) % det != 0 {
            return None;
        }

        let x = (c_x * b_y - c_y * b_x) / det;
        let y = (a_x * c_y - a_y * c_x) / det;

        Some(Vec2 { x, y })
    }
}

fn get_min_tokens(
    (button_a, button_b, prize): &(Vec2<u128>, Vec2<u128>, Vec2<u128>),
) -> Option<u128> {
    if let Some(solution) = solve(&(*button_a, *button_b, *prize)) {
        return Some(solution.x * 3 + solution.y);
    }
    if let Some(solution) = solve(&(*button_b, *button_a, *prize)) {
        return Some(solution.y * 3 + solution.x);
    }
    None
}

pub fn part_one(input: &str) -> Option<u128> {
    let res = parse(input, 0)
        .filter_map(|x| get_min_tokens(&x))
        .sum::<_>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u128> {
    let res = parse(input, 10000000000000)
        .filter_map(|x| get_min_tokens(&x))
        .sum::<_>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
