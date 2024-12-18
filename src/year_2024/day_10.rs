use std::collections::VecDeque;

use crate::utils::{Matrix2DBorrowed, Matrix2DOwned, Vector2};

pub fn part1(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut queue = VecDeque::with_capacity(grid.row_count() * grid.col_count());
    let mut visited = Matrix2DOwned::new(grid.row_count(), grid.col_count());

    let mut value = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let point = Vector2::new(c as isize, r as isize);
            if grid[point] != b'0' {
                continue;
            }
            
            visited[point] = true;
            queue.push_back(point);
            while let Some(pos) = queue.pop_front() {
                let current_val = grid[pos];
                if current_val == b'9' {
                    value += 1;
                }

                visited[pos] = false;

                for next_pos in pos.adjacent_points() {
                    if !grid.contains(next_pos) ||
                       grid[next_pos] != (current_val + 1) {

                        continue;
                    }

                    if visited[next_pos] {
                        continue;
                    }

                    visited[next_pos] = true;
                    queue.push_back(next_pos);
                }
            }
        }
    }

    value
}

pub fn part2(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut queue = VecDeque::with_capacity(grid.row_count() * grid.col_count());
    let mut visited_count = Matrix2DOwned::new(grid.row_count(), grid.col_count());

    let mut value = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let point = Vector2::new(c as isize, r as isize);
            if grid[point] != b'0' {
                continue;
            }

            queue.push_back(point);
            visited_count[point] = 1;
        }
    }

    while let Some(pos) = queue.pop_front() {
        let current_val = grid[pos];
        if current_val == b'9' {
            value += visited_count[pos];
            continue;
        }

        for next_pos in pos.adjacent_points() {
            if !grid.contains(next_pos) ||
                grid[next_pos] != (current_val + 1) {

                continue;
            }

            if visited_count[next_pos] == 0 {
                queue.push_back(next_pos);
            }

            visited_count[next_pos] += visited_count[pos];
        }
    }

    value
}
