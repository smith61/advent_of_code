
use std::collections::VecDeque;

use crate::{scaffold::InputParser, utils::{Grid, Grid2D, Point2D}};

const GRID_SIZE: usize = 71;

pub fn part1(mut input: InputParser) -> u64 {
    let mut bytes = Vec::new();
    while let Some(pair) = input.next_ints::<2>() {
        bytes.push(Point2D::new(pair[0], pair[1]));
    }

    let mut grid = Grid2D::<bool>::new(GRID_SIZE, GRID_SIZE);
    for index in 0..1024.min(bytes.len()) {
        grid[bytes[index]] = true;
    }

    let start_point = Point2D::new(0, 0);
    let end_point = Point2D::new((grid.row_count() - 1) as isize, (grid.col_count() - 1) as isize);
    let mut visited = Grid2D::new(grid.row_count(), grid.col_count());
    let mut queue = VecDeque::new();
    let mut next_queue = VecDeque::new();
    queue.push_back(start_point);
    visited[start_point] = true;
    for cost in 0.. {
        while let Some(point) = queue.pop_front() {
            if point == end_point {
                return cost;
            }

            for adj in point.adjacent_points() {
                if !grid.contains(adj) || grid[adj] {
                    continue;
                }

                if visited[adj] {
                    continue;
                }

                visited[adj] = true;
                next_queue.push_back(adj);
            }
        }

        assert!(!next_queue.is_empty());
        std::mem::swap(&mut queue, &mut next_queue);
    }

    unimplemented!();
}

pub fn part2(mut input: InputParser) -> String {
    let mut bytes = Vec::new();
    while let Some(pair) = input.next_ints::<2>() {
        bytes.push(Point2D::new(pair[0], pair[1]));
    }

    let mut grid = Grid2D::<bool>::new(GRID_SIZE, GRID_SIZE);
    let mut visited = Grid2D::<bool>::new(grid.row_count(), grid.col_count());
    let mut queue = VecDeque::new();
    for index in 0..bytes.len() {
        grid[bytes[index]] = true;
        if index != 0 {
            if !visited[bytes[index]] {
                continue;
            }

            visited.backing_store_mut().fill(false);
        }

        let start_point = Point2D::new(0, 0);
        let end_point = Point2D::new((grid.row_count() - 1) as isize, (grid.col_count() - 1) as isize);

        queue.clear();
        queue.push_back((start_point, 0));

        let mut found_exit = false;
        while let Some((point, cost)) = queue.pop_front() {
            if point == end_point {
                found_exit = true;
                break;
            }

            if visited[point] {
                continue;
            }

            visited[point] = true;
            for adj in point.adjacent_points() {
                if !grid.contains(adj) || grid[adj] {
                    continue;
                }

                queue.push_back((adj, cost + 1));
            }
        }

        if !found_exit {
            return format!("{},{}", bytes[index].x, bytes[index].y);
        }
    }

    unimplemented!();
}
