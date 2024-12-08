
use crate::utils::Point2D;

fn count_matches(grid: &[&[u8]], r: usize, c: usize, val: &[u8]) -> u64 {
    let mut count = 0;
    for r_d in -1isize..=1 {
        for r_c in -1isize..=1 {
            if r_d == 0 && r_c == 0 {
                continue;
            }

            let mut index = 0;
            let mut current_pos = Point2D::new(c as isize, r as isize);
            let pos_d = Point2D::new(r_c, r_d);
            loop {
                if current_pos.row() < 0 ||
                   current_pos.row_index() >= grid.len() ||
                   current_pos.column() < 0 ||
                   current_pos.column_index() >= grid[0].len() {

                    break;
                }

                if grid[current_pos.row_index()][current_pos.column_index()] != val[index] {
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

pub fn part1(input: &str) -> u64 {
    let mut count = 0;
    let grid =
        input.lines()
             .map(|l| l.as_bytes())
             .collect::<Vec<_>>();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            count += count_matches(&grid, r, c, b"XMAS");
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;
    let grid =
        input.lines()
             .map(|l| l.as_bytes())
             .collect::<Vec<_>>();

    for r in 1..grid.len()-1 {
        for c in 1..grid[r].len()-1 {
            if grid[r][c] != b'A' {
                continue;
            }

            let mut is_valid = false;
            if grid[r-1][c-1] == b'M' &&
               grid[r+1][c+1] == b'S' {

                is_valid = true;

            } else if grid[r-1][c-1] == b'S' &&
                      grid[r+1][c+1] == b'M' {

                is_valid = true;
            }

            if !is_valid {
                continue;
            }

            is_valid = false;
            if grid[r-1][c+1] == b'M' &&
               grid[r+1][c-1] == b'S' {
                
                is_valid = true;

            } else if grid[r-1][c+1] == b'S' &&
                      grid[r+1][c-1] == b'M' {
                
                is_valid = true;
            }

            if is_valid {
                count += 1;
            }
        }
    }

    count
}