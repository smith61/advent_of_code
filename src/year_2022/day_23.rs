
use crate::utils::Point2D;

use std::cmp;

use fxhash::FxHashSet;
use itertools::Itertools;

fn parse_input(input: &str) -> FxHashSet<Point2D> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == b'#')
            .map(move |(x, _)| {
                Point2D::new(x as isize, y as isize)
            })
        })
        .collect::<FxHashSet<_>>()
}

fn run_simulation<const MAX_ITERATIONS: u64>(positions: &mut FxHashSet<Point2D>) -> u64 {
    let mut search_order = [
        [(0, -1), (1, -1), (-1, -1)],
        [(0, 1), (1, 1), (-1, 1)],
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)]
    ];

    let mut iteration_count = 1;
    let mut next_positions = FxHashSet::with_capacity_and_hasher(positions.capacity(), Default::default());
    while iteration_count <= MAX_ITERATIONS {
        let mut moved_elf = false;
        for &current_pos in positions.iter() {
            let Point2D {x, y} = current_pos;

            let move_elf =
                (-1..=1).cartesian_product(-1..=1)
                .filter(|&(d_x, d_y)| d_x != 0 || d_y != 0)
                .any(|(d_x, d_y)| positions.contains(&Point2D::new(x + d_x, y + d_y)));

            moved_elf |= move_elf;

            let next_pos = if move_elf {
                let order =
                    search_order
                    .iter()
                    .find(|order| {
                        order
                        .iter()
                        .all(|(d_x, d_y)| {
                            !positions.contains(&Point2D::new(x + d_x, y + d_y))
                        })
                    });

                if let Some(order) = order {
                    Point2D::new(x + order[0].0, y + order[0].1)

                } else {
                    current_pos
                }

            } else {
                current_pos
            };

            if next_pos != current_pos && next_positions.contains(&next_pos) {
                next_positions.remove(&next_pos);
                next_positions.insert(current_pos);
                next_positions.insert(next_pos + (next_pos - current_pos));

            } else {
                next_positions.insert(next_pos);
            }
        }

        if !moved_elf {
            break;
        }

        std::mem::swap(positions, &mut next_positions);
        next_positions.clear();
        iteration_count += 1;
        search_order.rotate_left(1);
    }

    iteration_count
}

pub fn part1(input: &str) -> u64 {
    let mut positions = parse_input(input);
    run_simulation::<10>(&mut positions);

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    for &Point2D { x, y } in &positions {
        min_x = cmp::min(x, min_x);
        min_y = cmp::min(y, min_y);
        max_x = cmp::max(x, max_x);
        max_y = cmp::max(y, max_y);
    }

    ((max_x - min_x + 1) * (max_y - min_y + 1) - (positions.len() as isize)) as u64
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<{u64::MAX}>(&mut parse_input(input))
}