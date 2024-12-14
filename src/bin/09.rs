advent_of_code::solution!(9);

fn parse_disk(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(idx, n)| {
            if idx % 2 == 0 {
                vec![Some(idx / 2); n as usize]
            } else {
                vec![None; n as usize]
            }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut disk = parse_disk(input);

    while let (Some(first_none_index), Some(last_some_index)) = (
        disk.iter().position(|x| x.is_none()),
        disk.iter().rposition(|x| x.is_some()),
    ) {
        if first_none_index > last_some_index {
            break;
        }
        disk.swap(first_none_index, last_some_index);
    }

    let checksum = disk
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| x.as_ref().map(|z| idx as u128 * *z as u128))
        .sum::<u128>();

    Some(checksum as u128)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut disk = parse_disk(input);

    let mut r_pointer = disk.len() - 1;

    'main: loop {
        let id = disk[r_pointer];
        let mut r_len = 0;
        while disk[r_pointer] == id {
            r_len += 1;
            if let Some(rp) = r_pointer.checked_sub(1) {
                r_pointer = rp;
            } else {
                break 'main;
            }
        }

        let mut l_pointer = disk.iter().position(|x| x.is_none()).unwrap();
        if l_pointer > r_pointer {
            break 'main;
        }

        loop {
            let mut l_len = 0;
            while disk[l_pointer].is_some() {
                l_pointer += 1;
            }

            if l_pointer >= r_pointer {
                break;
            }

            while disk[l_pointer + l_len].is_none() {
                l_len += 1;
            }

            if l_len >= r_len {
                for i in 0..r_len {
                    disk[l_pointer + i] = id;
                    disk[r_pointer + 1 + i] = None;
                }
                break;
            }

            l_pointer += l_len;
        }

        while disk[r_pointer].is_none() {
            r_pointer -= 1;
        }
    }

    let checksum = disk
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| x.as_ref().map(|z| idx as u128 * *z as u128))
        .sum::<u128>();

    Some(checksum as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
