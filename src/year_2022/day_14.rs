use itertools::Itertools;

use ndarray::Array2;

use std::cmp;

struct PointIterator<'a> {
    bytes: &'a [u8]
}

impl<'a> PointIterator<'a> {

    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes
        }
    }

}

impl<'a> Iterator for PointIterator<'a> {

    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let mut x = 0;
        while self.bytes[0] != b',' {
            x = (x * 10) + ((self.bytes[0] - b'0') as usize);
            self.bytes = &self.bytes[1..];
        }
    
        self.bytes = &self.bytes[1..];
    
        let mut y = 0;
        while !self.bytes.is_empty() && self.bytes[0] != b' ' {
            y = (y * 10) + ((self.bytes[0] - b'0') as usize);
            self.bytes = &self.bytes[1..];
        }

        if !self.bytes.is_empty() {
            self.bytes = &self.bytes[4..];
        }

        Some((x, y))
    }
    
}

fn build_grid<const ADD_FLOOR: bool>(input: &str) -> (Array2<bool>, usize) {
    let mut x_min = usize::MAX;
    let mut y_min = 0;
    let mut x_max = usize::MIN;
    let mut y_max = usize::MIN;

    for line in input.lines() {
        for (x, y) in PointIterator::new(line.as_bytes()) {
            x_min = cmp::min(x_min, x);
            y_min = cmp::min(y_min, y);
            x_max = cmp::max(x_max, x);
            y_max = cmp::max(y_max, y);
        }
    }

    if ADD_FLOOR {
        y_max += 2;
        let floor_left = 500 - y_max - 1;
        let floor_right = 500 + y_max + 1;
        x_min = cmp::min(x_min, floor_left);
        x_max = cmp::max(x_max, floor_right);
    }
    
    let width = x_max - x_min + 1;
    let height = y_max - y_min + 1;
    let mut grid = Array2::from_elem((height, width), false);
    let x_offset = x_min;

    for line in input.lines() {
        for ((f_x, f_y), (s_x, s_y)) in PointIterator::new(line.as_bytes()).tuple_windows() {
            let x_min = cmp::min(f_x, s_x) - x_offset;
            let x_max = cmp::max(f_x, s_x) - x_offset;
            let y_min = cmp::min(f_y, s_y);
            let y_max = cmp::max(f_y, s_y);
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    grid[[y, x]] = true;
                }
            }
        }
    }

    if ADD_FLOOR {
        for x in 0..width {
            grid[[y_max, x]] = true;
        }
    }
    
    (grid, x_offset)
}

fn run_simulation<const ADD_FLOOR: bool>(input: &str) -> u64 {
    let (mut grid, x_offset) = build_grid::<ADD_FLOOR>(input);
    let mut sand_count = 0;
    let mut point_stack = Vec::with_capacity(grid.nrows());
    point_stack.push((0, 500 - x_offset));
    'outer: while let Some(mut point) = point_stack.pop() {
        'inner: loop {
            let (y, x) = point;
            if y + 1 >= grid.nrows() {
                break 'outer;
            }

            for new_point in [(y + 1, x), (y + 1, x - 1), (y + 1, x + 1)] {
                if !grid[new_point] {
                    point_stack.push(point);
                    point = new_point;
                    continue 'inner;
                }
            }

            break;
        }

        sand_count += 1;
        grid[point] = true;
    }

    sand_count
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<false>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<true>(input)
}