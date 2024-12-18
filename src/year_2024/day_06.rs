
use crate::utils::{Matrix, Matrix2DBorrowed, Matrix2DOwned, Vector2};

const D_V: [Vector2; 4] = [Vector2::new(0, -1), Vector2::new(1, 0), Vector2::new(0, 1), Vector2::new(-1, 0)];

fn can_escape<S: AsRef<[u8]>>(grid: &Matrix<2, S, u8>, mut current_pos: Vector2, mut direction: usize, visited_map: &mut Matrix2DOwned<u8>) -> bool {
    visited_map[current_pos] |= 1 << direction;
    loop {
        let next_pos = current_pos + D_V[direction];
        if !grid.contains(next_pos) {
            return true;
        }

        if grid[next_pos] == b'#' {
            direction = (direction + 1) % 4;
            continue;
        }

        if (visited_map[next_pos] & (1 << direction)) != 0 {
            return false;
        }

        visited_map[next_pos] |= 1 << direction;
        current_pos = next_pos;
    }
}

pub fn part1(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut current_pos = Vector2::new(0, 0);
    let mut direction = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let pos = Vector2::new(c as isize, r as isize);
            if grid[pos] == b'^' {
                current_pos = pos;
                direction = 0;
                break;

            } else if grid[pos] == b'>' {
                current_pos = pos;
                direction = 1;
                break;

            } else if grid[pos] == b'V' {
                current_pos = pos;
                direction = 2;
                break;

            } else if grid[pos] == b'<' {
                current_pos = pos;
                direction = 3;
                break;
            }
        }
    }

    let mut visited = Matrix2DOwned::new(grid.row_count(), grid.col_count());
    assert!(can_escape(&grid, current_pos, direction, &mut visited));
    visited.backing_store()
           .iter()
           .map(|r| (*r != 0) as u64)
           .sum()
}

pub fn part2(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut grid = grid.to_owned();

    let mut current_pos = Vector2::new(0, 0);
    let mut direction = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let pos = Vector2::new(c as isize, r as isize);
            if grid[pos] == b'^' {
                current_pos = pos;
                direction = 0;
                break;

            } else if grid[pos] == b'>' {
                current_pos = pos;
                direction = 1;
                break;

            } else if grid[pos] == b'V' {
                current_pos = pos;
                direction = 2;
                break;

            } else if grid[pos] == b'<' {
                current_pos = pos;
                direction = 3;
                break;
            }
        }
    }

    let mut visited = Matrix2DOwned::new(grid.row_count(), grid.col_count());
    let mut temp_visited = Matrix2DOwned::new(grid.row_count(), grid.col_count());
    let mut count = 0;
    loop {
        let next_pos = current_pos + D_V[direction];
        if !grid.contains(next_pos) {
            break;
        }

        if grid[next_pos] == b'#' {
            direction = (direction + 1) % 4;
            continue;
        }

        if visited[next_pos] == 0 {
            grid[next_pos] = b'#';
            temp_visited.backing_store_mut().copy_from_slice(visited.backing_store());
            if !can_escape(&grid, current_pos, direction, &mut temp_visited) {
                count += 1;
            }

            grid[next_pos] = b'.'
        }

        visited[next_pos] |= 1 << direction;
        current_pos = next_pos;
    }
    
    count
}