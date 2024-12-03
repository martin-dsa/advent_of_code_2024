advent_of_code::solution!(2);

fn is_safe(report: &Vec<u32>) -> bool {
    let mut pairs = report
        .as_slice()
        .windows(2)
        .map(|p| (p[0].cmp(&p[1]), (p[0] as i32 - p[1] as i32)));

    let (ord, diff) = pairs.next().unwrap();

    match ord {
        std::cmp::Ordering::Less => {
            if !(diff == -1 || diff == -2 || diff == -3) {
                return false;
            }
            pairs.all(|(o, d)| o == ord && (d == -1 || d == -2 || d == -3))
        }
        std::cmp::Ordering::Greater => {
            if !(diff == 1 || diff == 2 || diff == 3) {
                return false;
            }
            pairs.all(|(o, d)| o == ord && (d == 1 || d == 2 || d == 3))
        }
        std::cmp::Ordering::Equal => false,
    }
}

fn is_safe_2(report: &[u32]) -> bool {
    let mut copies = vec![report.to_owned().clone(); report.len()];
    for (i, copy) in copies.iter_mut().enumerate() {
        copy.remove(i);
    }
    copies.iter().any(is_safe)
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(is_safe)
        .count();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|report| is_safe_2(report))
        .count();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
