advent_of_code::solution!(4);

fn add_signed_to_usize(value: usize, offset: i32) -> Option<usize> {
    if offset >= 0 {
        value.checked_add(offset as usize)
    } else {
        value.checked_sub((-offset) as usize)
    }
}

fn get_next_letter(
    matrix: &[Vec<char>],
    start_index: (usize, usize),
    direction: (i32, i32),
    distance: i32,
) -> Option<&char> {
    add_signed_to_usize(start_index.0, direction.0 * distance)
        .and_then(|row_index| matrix.get(row_index))
        .and_then(|row| {
            add_signed_to_usize(start_index.1, direction.1 * distance)
                .and_then(|col_index| row.get(col_index))
        })
}

fn get_xmas(x_index: (usize, usize), matrix: &[Vec<char>]) -> u32 {
    let mut count = 0;

    for x_dir in -1i32..=1 {
        for y_dir in -1i32..=1 {
            if x_dir == 0 && y_dir == 0 {
                continue;
            }
            let mut xmas = "X".to_string();
            for distance in 1..=3 {
                if let Some(next_letter) =
                    get_next_letter(matrix, x_index, (x_dir, y_dir), distance)
                {
                    xmas.push(*next_letter);
                }
            }

            if xmas == "XMAS" {
                count += 1;
            }
        }
    }

    count
}

fn get_x_mas(a_index: (usize, usize), matrix: &[Vec<char>]) -> u32 {
    let mut xmas = "".to_string();

    for x_dir in -1i32..=1 {
        for y_dir in -1i32..=1 {
            if x_dir == 0 || y_dir == 0 {
                continue;
            }
            if let Some(next_letter) = get_next_letter(matrix, a_index, (x_dir, y_dir), 1) {
                xmas.push(*next_letter);
            }
        }
    }

    if xmas == "MMSS" || xmas == "MSMS" || xmas == "SSMM" || xmas == "SMSM" {
        1
    } else {
        0
    }
}

fn solve(
    input: &str,
    char_to_find: char,
    xmas_func: fn((usize, usize), &[Vec<char>]) -> u32,
) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let res = matrix
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            let value = matrix.clone();
            row.iter().enumerate().filter_map({
                move |(y, c)| {
                    if *c != char_to_find {
                        return None;
                    }
                    Some(xmas_func((x, y), &value))
                }
            })
        })
        .sum();
    Some(res)
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 'X', get_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 'A', get_x_mas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
