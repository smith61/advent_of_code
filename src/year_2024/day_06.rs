
use crate::utils::Point2D;

pub fn part1(input: &str) -> u64 {
    let grid =
        input.lines()
             .map(|l| l.as_bytes())
             .collect::<Vec<_>>();

    let d_v = [Point2D::new(0, -1), Point2D::new(1, 0), Point2D::new(0, 1), Point2D::new(-1, 0)];

    let mut current_pos = Point2D::new(0, 0);
    let mut direction = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'^' {
                current_pos = Point2D::new(c as isize, r as isize);
                direction = 0;

            } else if grid[r][c] == b'>' {
                current_pos = Point2D::new(c as isize, r as isize);
                direction = 1;

            } else if grid[r][c] == b'V' {
                current_pos = Point2D::new(c as isize, r as isize);
                direction = 2;

            } else if grid[r][c] == b'<' {
                current_pos = Point2D::new(c as isize, r as isize);
                direction = 3;
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[current_pos.row_index()][current_pos.column_index()] = true;

    loop {
        let next_pos = current_pos + d_v[direction];
        if next_pos.row() < 0 ||
           next_pos.row_index() >= grid.len() ||
           next_pos.column() < 0 ||
           next_pos.column_index() >= grid[0].len() {

            break;
        }

        if grid[next_pos.row_index()][next_pos.column_index()] == b'#' {
            direction = (direction + 1) % 4;
            continue;
        }

        visited[next_pos.row_index()][next_pos.column_index()] = true;
        current_pos = next_pos;
    }
    
    visited.iter()
           .map(|r| r.iter().map(|v| *v as u64).sum::<u64>())
           .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut grid =
        input.lines()
             .map(|l| l.as_bytes().to_owned())
             .collect::<Vec<_>>();

    let d_v = [Point2D::new(0, -1), Point2D::new(1, 0), Point2D::new(0, 1), Point2D::new(-1, 0)];

    let mut starting_pos = Point2D::new(0, 0);
    let mut starting_direction = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'^' {
                starting_pos = Point2D::new(c as isize, r as isize);
                starting_direction = 0;

            } else if grid[r][c] == b'>' {
                starting_pos = Point2D::new(c as isize, r as isize);
                starting_direction = 1;

            } else if grid[r][c] == b'V' {
                starting_pos = Point2D::new(c as isize, r as isize);
                starting_direction = 2;

            } else if grid[r][c] == b'<' {
                starting_pos = Point2D::new(c as isize, r as isize);
                starting_direction = 3;
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    {
        let mut current_pos = starting_pos;
        let mut direction = starting_direction;
        loop {
            let next_pos = current_pos + d_v[direction];
            if next_pos.row() < 0 ||
               next_pos.row_index() >= grid.len() ||
               next_pos.column() < 0 ||
               next_pos.column_index() >= grid[0].len() {

                break;
            }

            if grid[next_pos.row_index()][next_pos.column_index()] == b'#' {
                direction = (direction + 1) % 4;
                continue;
            }

            visited[next_pos.row_index()][next_pos.column_index()] = true;
            current_pos = next_pos;
        }
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let old_value = grid[r][c];
            if old_value != b'.' &&
               !visited[r][c] {

                continue;
            }

            grid[r][c] = b'#';

            let mut current_pos = starting_pos;
            let mut direction = starting_direction;

            let mut visited = vec![vec![0; grid[0].len()]; grid.len()];
            loop {
                if (visited[current_pos.row_index()][current_pos.column_index()] & (1 << direction)) != 0 {
                    count += 1;
                    break;
                }

                visited[current_pos.row_index()][current_pos.column_index()] |= 1 << direction;

                let next_pos = current_pos + d_v[direction];
                if next_pos.row() < 0 ||
                   next_pos.row_index() >= grid.len() ||
                   next_pos.column() < 0 ||
                   next_pos.column_index() >= grid[0].len() {

                    break;
                }

                if grid[next_pos.row_index()][next_pos.column_index()] == b'#' {
                    direction = (direction + 1) % 4;
                    continue;
                }

                current_pos = next_pos;
            }

            grid[r][c] = old_value;
        }
    }
    
    count
}