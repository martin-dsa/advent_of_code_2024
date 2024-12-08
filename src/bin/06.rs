use std::{collections::HashSet, sync::Arc, thread};

use advent_of_code::get_next_letter;

advent_of_code::solution!(6);

static GUARD: char = '^';
static OBSTRUCTION: char = '#';

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

fn get_visited(grid: &[Vec<char>]) -> Option<u32> {
    let mut position = grid
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .position(|&ch| ch == GUARD)
                .map(|col_index| (row_index, col_index))
        })
        .unwrap();
    let mut direction = (-1, 0);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(position);
    let mut visited_turns: HashSet<(usize, usize)> = HashSet::new();
    let mut turned = false;

    while let Some(next_char) = get_next_letter(grid, position, direction, 1) {
        if next_char.character == OBSTRUCTION {
            if !turned && visited_turns.contains(&position) {
                return None;
            }
            turned = true;
            direction = turn_right(direction);
            visited_turns.insert(position);
            continue;
        }
        turned = false;
        position = next_char.position;
        visited.insert(position);
    }

    Some(visited.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    get_visited(&grid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let grid = Arc::new(grid);

    let mut handles = Vec::<std::thread::JoinHandle<u32>>::new();

    let len = grid.len();

    for i in 0..len {
        let grid = Arc::clone(&grid);
        let handle = thread::spawn(move || {
            let mut loop_count = 0;
            for j in 0..len {
                let char_to_replace_with_obstruction = grid[i][j];
                if char_to_replace_with_obstruction == OBSTRUCTION
                    || char_to_replace_with_obstruction == GUARD
                {
                    continue;
                }
                let mut temp_grid = (*grid).clone();
                temp_grid[i][j] = OBSTRUCTION;
                let visited = get_visited(&temp_grid);
                if visited.is_none() {
                    loop_count += 1;
                }
            }
            loop_count
        });
        handles.push(handle);
    }
    let res = handles
        .into_iter()
        .map(|jh| jh.join().unwrap())
        .sum::<u32>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
