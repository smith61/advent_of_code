use std::collections::VecDeque;

use crate::utils::{Grid, Grid2D, Grid2DBorrowed, Point2D};

fn get_cost(starting_point: Point2D, grid: &impl Grid<Point2D, Output = u8>, visited: &mut Grid2D<bool>) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back(starting_point);

    let mut area = 0;
    let mut perimeter = 0;
    visited[starting_point] = true;
    while let Some(point) = queue.pop_front() {
        area += 1;
        for adj in point.adjacent_points() {
            if !grid.contains(adj) {
                perimeter += 1;
                continue;
            }

            if grid[adj] == grid[point] {
                if !visited[adj] {
                    visited[adj] = true;
                    queue.push_back(adj);
                }

            } else {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}

fn matches(i: Point2D, j: Point2D, grid: &impl Grid<Point2D, Output = u8>) -> bool {
    grid.contains(i) &&
    grid.contains(j) &&
    grid[i] == grid[j]
}

fn corner_count(point: Point2D, grid: &impl Grid<Point2D, Output = u8>) -> u64 {
    let mut num_corners = 0;

    const UP: Point2D = Point2D::new(0, -1);
    const RIGHT: Point2D = Point2D::new(1, 0);
    const DOWN: Point2D = Point2D::new(0, 1);
    const LEFT: Point2D = Point2D::new(-1, 0);

    if matches(point, point + UP, grid) &&
       matches(point, point + LEFT, grid) &&
       !matches(point, point + UP + LEFT, grid) {

        num_corners += 1;
    }

    if matches(point, point + UP, grid) &&
       matches(point, point + RIGHT, grid) &&
       !matches(point, point + UP + RIGHT, grid) {

        num_corners += 1;
    }

    if matches(point, point + DOWN, grid) &&
       matches(point, point + RIGHT, grid) &&
       !matches(point, point + DOWN + RIGHT, grid) {

        num_corners += 1;
    }

    if matches(point, point + DOWN, grid) &&
       matches(point, point + LEFT, grid) &&
       !matches(point, point + DOWN + LEFT, grid) {

        num_corners += 1;
    }

    if !matches(point, point + UP, grid) &&
       !matches(point, point + LEFT, grid) {
        
        num_corners += 1;
    }

    if !matches(point, point + UP, grid) &&
       !matches(point, point + RIGHT, grid) {
        
        num_corners += 1;
    }

    if !matches(point, point + DOWN, grid) &&
       !matches(point, point + LEFT, grid) {
        
        num_corners += 1;
    }

    if !matches(point, point + DOWN, grid) &&
       !matches(point, point + RIGHT, grid) {
        
        num_corners += 1;
    }

    num_corners
}

fn get_cost2(starting_point: Point2D, grid: &impl Grid<Point2D, Output = u8>, visited: &mut Grid2D<bool>) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back(starting_point);

    let mut area = 0;
    let mut side_count = 0;
    visited[starting_point] = true;
    while let Some(point) = queue.pop_front() {
        area += 1;
        side_count += corner_count(point, grid);
        for adj in point.adjacent_points() {
            if !grid.contains(adj) {
                continue;
            }

            if grid[adj] == grid[point] {
                if !visited[adj] {
                    visited[adj] = true;
                    queue.push_back(adj);
                }

            }
        }
    }

    area * side_count
}

pub fn part1(input: &str) -> u64 {
    let grid = Grid2DBorrowed::from_input_lines(input);
    let mut visited = Grid2D::new(grid.row_count(), grid.col_count());

    let mut cost = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let point = Point2D::new(c as isize, r as isize);
            if visited[point] {
                continue;
            }

            cost += get_cost(point, &grid, &mut visited);
        }
    }
    
    cost
}

pub fn part2(input: &str) -> u64 {
    let grid = Grid2DBorrowed::from_input_lines(input);
    let mut visited = Grid2D::new(grid.row_count(), grid.col_count());
    
    let mut cost = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let point = Point2D::new(c as isize, r as isize);
            if visited[point] {
                continue;
            }

            cost += get_cost2(point, &grid, &mut visited);
        }
    }

    cost
}
