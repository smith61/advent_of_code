use std::collections::VecDeque;

use crate::utils::{Grid, Grid2D, Grid2DBorrowed, Point2D};

fn get_number_of_corners(point: Point2D, grid: &impl Grid<Point2D, Output = u8>) -> u64 {
    let mut number_of_corners = 0;

    let mut matches = [[false; 3]; 3];

    let value = grid[point];
    for r_d in 0..3 {
        for c_d in 0..3 {
            let compare_point = point + Point2D::new(c_d as isize - 1, r_d as isize - 1);
            matches[r_d][c_d] = grid.contains(compare_point) && grid[compare_point] == value;
        }
    }

    if matches[0][1] {
        if matches[1][0] &&
           !matches[0][0] {

            number_of_corners += 1;
        }

        if matches[1][2] &&
           !matches[0][2] {

            number_of_corners += 1;
        }

    } else {
        if !matches[1][0] {
            number_of_corners += 1;
        }

        if !matches[1][2] {
            number_of_corners += 1;
        }
    }

    if matches[2][1] {
        if matches[1][2] &&
           !matches[2][2] {

            number_of_corners += 1;
        }

        if matches[1][0] &&
           !matches[2][0] {

            number_of_corners += 1;
        }

    } else {
        if !matches[1][0] {
            number_of_corners += 1;
        }

        if !matches[1][2] {
            number_of_corners += 1;
        }
    }

    number_of_corners
}

fn solve<const COUNT_PERIMETER: bool>(input: &str) -> u64 {
    let grid = Grid2DBorrowed::from_input_lines(input);
    let mut visited = Grid2D::new(grid.row_count(), grid.col_count());
    let mut queue = VecDeque::with_capacity(32);

    let mut cost = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let starting_point = Point2D::new(c as isize, r as isize);
            if visited[starting_point] {
                continue;
            }

            let starting_value = grid[starting_point];

            let mut area = 0;
            let mut edge_cost = 0;
            queue.push_back(starting_point);
            visited[starting_point] = true;
            while let Some(current_point) = queue.pop_front() {
                area += 1;
                if !COUNT_PERIMETER {
                    edge_cost += get_number_of_corners(current_point, &grid);
                }

                for adjacent_point in current_point.adjacent_points() {
                    if !grid.contains(adjacent_point) {
                        if COUNT_PERIMETER {
                            edge_cost += 1;
                        }

                    } else if grid[adjacent_point] == starting_value {
                        if !visited[adjacent_point] {
                            visited[adjacent_point] = true;
                            queue.push_back(adjacent_point);
                        }

                    } else if COUNT_PERIMETER {
                        edge_cost += 1;
                    }
                }
            }

            cost += area * edge_cost;
        }
    }

    cost
}

pub fn part1(input: &str) -> u64 {
    solve::<true>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<false>(input)
}
