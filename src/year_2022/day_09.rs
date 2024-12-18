use crate::utils::Vector2;

use fxhash::FxHashSet;

fn run_simulation<const KNOT_COUNT: usize>(input: &str) -> u64 {
    let mut knots = [Vector2::default(); KNOT_COUNT];
    let mut visited_set = FxHashSet::default();
    for line in input.lines() {
        let as_bytes = line.as_bytes();
        let dir_vec = match as_bytes[0] {
            b'R' => Vector2::new(1, 0),
            b'L' => Vector2::new(-1, 0),
            b'U' => Vector2::new(0, -1),
            b'D' => Vector2::new(0, 1),
            _ => unreachable!()
        };

        let mut distance = 0;
        for index in 2..as_bytes.len() {
            distance = (distance * 10) + ((as_bytes[index] - b'0') as u64);
        }

        for _ in 0..distance {
            knots[0] += dir_vec;
            for index in 1..KNOT_COUNT {
                let head = knots[index - 1];
                let tail = knots[index];

                let x_dist = head.x() - tail.x();
                let y_dist = head.y() - tail.y();
                if x_dist.abs() >= 2 || y_dist.abs() >= 2 {
                    let dir_vec = Vector2::new(x_dist.signum(), y_dist.signum());
                    knots[index] += dir_vec;
                }
            }

            visited_set.insert(knots[KNOT_COUNT - 1]);
        }
    }

    visited_set.len() as u64
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<2>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<10>(input)
}