use crate::utils::Point2D;

use std::collections::VecDeque;

use fxhash::FxHashSet;
use itertools::Itertools;

fn get_height(b: u8) -> u8 {
    if b == b'E' {
        b'z' - b'a'

    } else if b == b'S' {
        b'a' - b'a'

    } else {
        b - b'a'
    }
}

fn run_bfs<const END_GOAL: u8>(input: &str) -> u64 {
    let grid =
        input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let row_count = grid.len();
    let col_count = grid[0].len();

    let start_index =
        (0usize..row_count)
        .cartesian_product(0usize..col_count)
        .find(|(row, col)| grid[*row][*col] == b'E')
        .map(|(row, col)| Point2D::new(col as isize, row as isize))
        .unwrap();

    let mut current_queue = VecDeque::new();
    let mut next_queue = VecDeque::new();
    let mut visited = FxHashSet::default();
    let mut distance = 0;

    current_queue.push_back(start_index);
    'outer: loop {
        for point in current_queue.drain(..) {
            let val = grid[point.row() as usize][point.column() as usize];
            if val == END_GOAL {
                break 'outer;
            }

            let height = get_height(val);
            if height == END_GOAL {
                break 'outer;
            }

            for dir_vec in [Point2D::new(1, 0), Point2D::new(-1, 0), Point2D::new(0, 1), Point2D::new(0, -1)] {
                let new_point = point + dir_vec;
                if new_point.row() < 0 ||
                   new_point.row() >= row_count as isize ||
                   new_point.column() < 0 ||
                   new_point.column() >= col_count as isize {

                    continue;
                }

                if visited.contains(&new_point) {
                    continue;
                }

                let new_height = get_height(grid[new_point.row() as usize][new_point.column() as usize]);
                if (new_height + 1) >= height {    
                    visited.insert(new_point);
                    next_queue.push_back(new_point);
                } 
            }
        }

        distance += 1;
        std::mem::swap(&mut current_queue, &mut next_queue);
    }

    distance
}

pub fn part1(input: &str) -> u64 {
    run_bfs::<b'S'>(input)
}

pub fn part2(input: &str) -> u64 {
    run_bfs::<b'a'>(input)
}