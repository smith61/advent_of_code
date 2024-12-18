
use crate::utils::{Matrix2DBorrowed, Vector2};

fn count_matches(grid: &Matrix2DBorrowed<u8>, r: usize, c: usize, val: &[u8]) -> u64 {
    let mut count = 0;
    for r_d in -1isize..=1 {
        for r_c in -1isize..=1 {
            if r_d == 0 && r_c == 0 {
                continue;
            }

            let mut index = 0;
            let mut current_pos = Vector2::new(c as isize, r as isize);
            let pos_d = Vector2::new(r_c, r_d);
            loop {
                if !grid.contains(current_pos) {
                    break;
                }

                if grid[current_pos] != val[index] {
                    break;
                }

                index += 1;
                if index >= val.len() {
                    count += 1;
                    break;
                }

                current_pos += pos_d;
            }
        }
    }

    count
}

pub fn part1(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut count = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            count += count_matches(&grid, r, c, b"XMAS");
        }
    }

    count
}

pub fn part2(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut count = 0;

    const UP_LEFT: Vector2 = Vector2::new(-1, -1);
    const UP_RIGHT: Vector2 = Vector2::new(1, -1);
    const DOWN_LEFT: Vector2 = Vector2::new(-1, 1);
    const DOWN_RIGHT: Vector2 = Vector2::new(1, 1);
    for r in 1..grid.row_count()-1 {
        for c in 1..grid.col_count()-1 {
            let pos = Vector2::new(c as isize, r as isize);
            if grid[pos] != b'A' {
                continue;
            }

            let mut is_valid = false;
            if grid[pos + UP_LEFT] == b'M' &&
               grid[pos + DOWN_RIGHT] == b'S' {

                is_valid = true;

            } else if grid[pos + UP_LEFT] == b'S' &&
                      grid[pos + DOWN_RIGHT] == b'M' {

                is_valid = true;
            }

            if !is_valid {
                continue;
            }

            is_valid = false;
            if grid[pos + UP_RIGHT] == b'M' &&
               grid[pos + DOWN_LEFT] == b'S' {
                
                is_valid = true;

            } else if grid[pos + UP_RIGHT] == b'S' &&
                      grid[pos + DOWN_LEFT] == b'M' {
                
                is_valid = true;
            }

            if is_valid {
                count += 1;
            }
        }
    }

    count
}