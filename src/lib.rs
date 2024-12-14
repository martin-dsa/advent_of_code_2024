use num::Num;

pub mod template;

// Use this file to add helper functions and additional modules.

pub fn add_signed_to_usize(value: usize, offset: i32) -> Option<usize> {
    if offset >= 0 {
        value.checked_add(offset as usize)
    } else {
        value.checked_sub((-offset) as usize)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct CharWithIndex<T> {
    pub value: T,
    pub position: (usize, usize),
}

pub fn get_next_cell<T: Copy>(
    matrix: &[Vec<T>],
    start_index: (usize, usize),
    direction: (i32, i32),
    distance: i32,
) -> Option<CharWithIndex<T>> {
    add_signed_to_usize(start_index.0, direction.0 * distance)
        .and_then(|row_index| matrix.get(row_index).map(|row| (row, row_index)))
        .and_then(|(row, row_index)| {
            add_signed_to_usize(start_index.1, direction.1 * distance).and_then(|col_index| {
                row.get(col_index).map(|c| CharWithIndex {
                    value: *c,
                    position: (row_index, col_index),
                })
            })
        })
}

pub fn are_collinear(p1: &(i32, i32), p2: &(i32, i32), p3: &(i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    // Check if the determinant is zero
    x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2) == 0
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vec2<T: Num> {
    pub x: T,
    pub y: T,
}
