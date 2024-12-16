use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}};

use crate::utils::{Grid, Grid2D, Grid2DBorrowed, Point2D};

const LEFT: Point2D = Point2D::new(-1, 0);
const RIGHT: Point2D = Point2D::new(1, 0);
const UP: Point2D = Point2D::new(0, -1);
const DOWN: Point2D = Point2D::new(0, 1);

const DIRECTIONS: [Point2D; 4] = [
    RIGHT,
    DOWN,
    LEFT,
    UP
];

fn solve<const PART1: bool>(input_grid: Grid2DBorrowed) -> u64 {
    let mut starting_position = Point2D::default();
    let mut ending_position = Point2D::default();

    for r in 0..input_grid.row_count() {
        for c in 0..input_grid.col_count() {
            let point = Point2D::new(c as isize, r as isize);
            if input_grid[point] == b'S' {
                starting_position = point;
            } else if input_grid[point] == b'E' {
                ending_position = point;
            }
        }
    }

    let mut grid_costs: Grid2D<[u64; 4]> = Grid2D::new(input_grid.row_count(), input_grid.col_count());
    grid_costs.backing_store_mut().fill([u64::MAX; 4]);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, starting_position, 0)));

    let mut minimum_cost = u64::MAX;
    while let Some(Reverse((cost, position, direction))) = queue.pop() {
        if grid_costs[position][direction] != u64::MAX {
            assert!(grid_costs[position][direction] <= cost);
            continue;
        }

        grid_costs[position][direction] = cost;
        if position == ending_position {
            if PART1 {
                return cost;
            }

            minimum_cost = minimum_cost.min(cost);
            continue;
        }

        if cost > minimum_cost {
            continue;
        }

        let next_position = position + DIRECTIONS[direction];
        if input_grid[next_position] != b'#' {
            queue.push(Reverse((cost + 1, next_position, direction)));
        }

        queue.push(Reverse((cost + 1000, position, (direction + 1) % 4)));
        queue.push(Reverse((cost + 1000, position, (direction + 3) % 4)));
    }

    assert!(!PART1);

    let mut backtrack = VecDeque::new();
    for direction in 0..4 {
        if grid_costs[ending_position][direction] == minimum_cost {
            backtrack.push_back((minimum_cost, ending_position, direction));
        }
    }

    let mut visited: Grid2D<bool> = Grid2D::new(input_grid.row_count(), input_grid.col_count());
    let mut visited_count = 0;
    while let Some((minimum_cost, position, direction)) = backtrack.pop_front() {
        assert_eq!(grid_costs[position][direction], minimum_cost);

        if !visited[position] {
            visited[position] = true;
            visited_count += 1;
        }

        let previous_position = position - DIRECTIONS[direction];
        if minimum_cost >= 1 {
            if grid_costs[previous_position][direction] == (minimum_cost - 1) {
                backtrack.push_back((minimum_cost - 1, previous_position, direction));
            }
        }

        if minimum_cost >= 1000 {
            if grid_costs[position][(direction + 1) % 4] == (minimum_cost - 1000) {
                backtrack.push_back((minimum_cost - 1000, position, (direction + 1) % 4));
            }

            if grid_costs[position][(direction + 3) % 4] == (minimum_cost - 1000) {
                backtrack.push_back((minimum_cost - 1000, position, (direction + 3) % 4));
            }
        }
    }

    visited_count
}

pub fn part1(input_grid: Grid2DBorrowed) -> u64 {
    solve::<true>(input_grid)
}

pub fn part2(input_grid: Grid2DBorrowed) -> u64 {
    solve::<false>(input_grid)
}
