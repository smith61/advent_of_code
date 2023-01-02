
use crate::utils::Point2D;

use fxhash::FxHashSet;

fn run_simulation<const PART_COUNT: usize>(input: &str) -> u64 {
    let mut visited_set = FxHashSet::default();
    let mut current_positions = [Point2D::new(0, 0); PART_COUNT];
    visited_set.insert(current_positions[0]);

    let mut pos_index = 0;
    for &b in input.trim().as_bytes() {
        let dir = match b {
            b'>' => Point2D::new(1, 0),
            b'<' => Point2D::new(-1, 0),
            b'v' => Point2D::new(0, 1),
            b'^' => Point2D::new(0, -1),
            _ => unreachable!()
        };

        current_positions[pos_index] += dir;
        visited_set.insert(current_positions[pos_index]);
        pos_index = (pos_index + 1) % PART_COUNT;
    }

    visited_set.len() as u64
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<1>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<2>(input)
}