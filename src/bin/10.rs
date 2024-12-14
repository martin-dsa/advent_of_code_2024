use advent_of_code::{get_next_cell, CharWithIndex};
use itertools::Itertools;
use lazy_static::lazy_static;

advent_of_code::solution!(10);
lazy_static! {
    static ref dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
}

fn get_trail(
    grid: &[Vec<u32>],
    idx: (usize, usize),
    res: &mut Vec<Vec<CharWithIndex<u32>>>,
) -> Vec<Vec<CharWithIndex<u32>>> {
    let v = grid[idx.0][idx.1];
    for x in res.iter_mut() {
        x.push(CharWithIndex {
            position: idx,
            value: v,
        });
    }
    if v == 9 {
        return res.to_owned();
    }
    dirs.iter()
        .filter_map(|i| get_next_cell(grid, idx, *i, 1).filter(|&c| c.value == v + 1))
        .flat_map(move |x| get_trail(grid, x.position, &mut res.clone()))
        .filter(|x| x.len() == 10)
        .collect_vec()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn solve(input: &str, unique: bool) -> Option<u32> {
    let grid = parse(input);

    let res = grid
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            let grid_ref = &grid;
            row.iter().enumerate().filter_map(move |(col_idx, n)| {
                if *n != 0 {
                    None
                } else {
                    let trails = get_trail(grid_ref, (row_idx, col_idx), &mut vec![vec![]]);

                    if unique {
                        return Some(trails.iter().unique_by(|x| x.last()).count());
                    }

                    Some(trails.len())
                }
            })
        })
        .sum::<usize>();
    Some(res as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
