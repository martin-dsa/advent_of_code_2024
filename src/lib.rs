pub mod template;

// Use this file to add helper functions and additional modules.

pub fn add_signed_to_usize(value: usize, offset: i32) -> Option<usize> {
    if offset >= 0 {
        value.checked_add(offset as usize)
    } else {
        value.checked_sub((-offset) as usize)
    }
}
pub struct CharWithIndex {
    pub character: char,
    pub position: (usize, usize),
}
pub fn get_next_letter(
    matrix: &[Vec<char>],
    start_index: (usize, usize),
    direction: (i32, i32),
    distance: i32,
) -> Option<CharWithIndex> {
    add_signed_to_usize(start_index.0, direction.0 * distance)
        .and_then(|row_index| matrix.get(row_index).map(|row| (row, row_index)))
        .and_then(|(row, row_index)| {
            add_signed_to_usize(start_index.1, direction.1 * distance).and_then(|col_index| {
                row.get(col_index).map(|c| CharWithIndex {
                    character: *c,
                    position: (row_index, col_index),
                })
            })
        })
}
