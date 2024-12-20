use std::collections::VecDeque;

use crate::utils::{DijkstraQueue, Matrix2DBorrowed, Matrix2DOwned, Vector2};

const LEFT: Vector2 = Vector2::new(-1, 0);
const RIGHT: Vector2 = Vector2::new(1, 0);
const UP: Vector2 = Vector2::new(0, -1);
const DOWN: Vector2 = Vector2::new(0, 1);

const DIRECTIONS: [Vector2; 4] = [
    RIGHT,
    DOWN,
    LEFT,
    UP
];

fn solve<const PART1: bool>(input_grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut starting_position = Vector2::default();
    let mut ending_position = Vector2::default();

    for r in 0..input_grid.row_count() {
        for c in 0..input_grid.col_count() {
            let point = Vector2::new(c as isize, r as isize);
            if input_grid[point] == b'S' {
                starting_position = point;
            } else if input_grid[point] == b'E' {
                ending_position = point;
            }
        }
    }

    let mut grid_costs: Matrix2DOwned<[u64; 4]> = Matrix2DOwned::new(input_grid.row_count(), input_grid.col_count());
    grid_costs.backing_store_mut().fill([u64::MAX; 4]);

    let mut queue = DijkstraQueue::new();
    queue.push(0, (starting_position, 0));

    let mut minimum_cost = u64::MAX;
    while let Some((cost, (position, direction))) = queue.pop() {
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

        if cost >= minimum_cost {
            continue;
        }

        let next_position = position + DIRECTIONS[direction];
        if input_grid[next_position] != b'#' {
            if grid_costs[next_position][direction] >= (cost + 1) {
                queue.push(cost + 1, (next_position, direction));
            }
        }

        if grid_costs[position][(direction + 1) % 4] >= (cost + 1000) {
            queue.push(cost + 1000, (position, (direction + 1) % 4));
        }
        
        if grid_costs[position][(direction + 3) % 4] >= (cost + 1000) {
            queue.push(cost + 1000, (position, (direction + 3) % 4));
        }
    }

    assert!(!PART1);

    let mut backtrack = VecDeque::new();
    for direction in 0..4 {
        if grid_costs[ending_position][direction] == minimum_cost {
            backtrack.push_back((minimum_cost, ending_position, direction));
        }
    }

    let mut visited: Matrix2DOwned<bool> = Matrix2DOwned::new(input_grid.row_count(), input_grid.col_count());
    let mut visited_count = 0;
    while let Some((minimum_cost, position, direction)) = backtrack.pop_front() {
        if grid_costs[position][direction] == u64::MAX {
            continue;
        }

        assert_eq!(grid_costs[position][direction], minimum_cost);
        grid_costs[position][direction] = u64::MAX;

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

pub fn part1(input_grid: Matrix2DBorrowed<u8>) -> u64 {
    solve::<true>(input_grid)
}

pub fn part2(input_grid: Matrix2DBorrowed<u8>) -> u64 {
    solve::<false>(input_grid)
}
