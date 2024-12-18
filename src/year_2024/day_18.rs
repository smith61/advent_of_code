
use std::collections::VecDeque;

use crate::{scaffold::InputParser, utils::{Matrix2DOwned, Vector2}};

const GRID_SIZE: usize = 71;
const LEFT: Vector2 = Vector2::new(-1, 0);
const UP: Vector2 = Vector2::new(0, -1);

pub fn part1(mut input: InputParser) -> u64 {
    let mut bytes = Vec::new();
    while let Some(pair) = input.next_ints::<2>() {
        bytes.push(Vector2::new(pair[0], pair[1]));
    }

    let mut grid = Matrix2DOwned::<bool>::new(GRID_SIZE, GRID_SIZE);
    for index in 0..1024.min(bytes.len()) {
        grid[bytes[index]] = true;
    }

    let start_point = Vector2::new(0, 0);
    let end_point = Vector2::new((grid.row_count() - 1) as isize, (grid.col_count() - 1) as isize);
    let mut visited = Matrix2DOwned::new(grid.row_count(), grid.col_count());
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

    unreachable!();
}

struct UnionFindGrid {
    backing_grid: Matrix2DOwned<usize>
}

impl UnionFindGrid {

    pub fn new(row_count: usize, col_count: usize) -> Self {
        let mut this = Self {
            backing_grid: Matrix2DOwned::new(row_count, col_count)
        };

        for r in 0..row_count {
            for c in 0..col_count {
                let point = Vector2::new(c as isize, r as isize);
                this.backing_grid[point] = this.point_to_index(point);
            }
        }

        this
    }

    pub fn add_relation(&mut self, point_1: Vector2, point_2: Vector2) {
        let point_1_root = self.get_root(point_1);
        let point_2_root = self.get_root(point_2);
        if point_1_root != point_2_root {
            let orig_root = self.index_to_point(point_1_root);
            self.backing_grid[orig_root] = point_2_root;
        }
    }

    pub fn get_root(&mut self, point: Vector2) -> usize {
        if self.backing_grid[point] != self.point_to_index(point) {
            let parent = self.index_to_point(self.backing_grid[point]);
            let root = self.get_root(parent);
            self.backing_grid[point] = root;
        }

        self.backing_grid[point]
    }

    fn point_to_index(&self, point: Vector2) -> usize {
        assert!(self.backing_grid.contains(point));

        point.row_index() * self.backing_grid.col_count() + point.column_index()
    }

    fn index_to_point(&self, index: usize) -> Vector2 {
        let row_index = index / self.backing_grid.col_count();
        let col_index = index % self.backing_grid.col_count();

        Vector2::new(col_index as isize, row_index as isize)
    }

}

pub fn part2(mut input: InputParser) -> Vector2 {
    let mut corrupted_bytes = Vec::new();
    let mut corrupted_bytes_grid = Matrix2DOwned::new(GRID_SIZE, GRID_SIZE);
    while let Some(pair) = input.next_ints::<2>() {
        let point = Vector2::new(pair[0], pair[1]);
        corrupted_bytes.push(point);
        corrupted_bytes_grid[point] = true;
    }

    let mut grid = UnionFindGrid::new(GRID_SIZE, GRID_SIZE);
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            let grid_point = Vector2::new(c as isize, r as isize);
            if corrupted_bytes_grid[grid_point] {
                continue;
            }

            let left_point = grid_point + LEFT;
            if corrupted_bytes_grid.contains(left_point) &&
               !corrupted_bytes_grid[left_point] {

                grid.add_relation(grid_point, left_point);
            }

            let up_point = grid_point + UP;
            if corrupted_bytes_grid.contains(up_point) &&
               !corrupted_bytes_grid[up_point] {

                grid.add_relation(grid_point, up_point);
            }
        }
    }

    let start_point = Vector2::new(0, 0);
    let end_point = Vector2::new((GRID_SIZE - 1) as isize, (GRID_SIZE - 1) as isize);

    assert_ne!(grid.get_root(start_point), grid.get_root(end_point));

    for index in (0..corrupted_bytes.len()).rev() {
        let corrupted_byte = corrupted_bytes[index];
        corrupted_bytes_grid[corrupted_byte] = false;
        for adjacent in corrupted_byte.adjacent_points() {
            if !corrupted_bytes_grid.contains(adjacent) ||
                corrupted_bytes_grid[adjacent] {

                continue;
            }

            grid.add_relation(corrupted_byte, adjacent);
        }

        if grid.get_root(start_point) == grid.get_root(end_point) {
            return corrupted_byte;
        }
    }

    unreachable!();
}
