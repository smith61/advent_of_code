
use crate::utils::Vector2;

use std::cmp;

use fxhash::FxHashSet;
use itertools::Itertools;

fn parse_input(input: &str) -> FxHashSet<Vector2> {
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
                Vector2::new(x as isize, y as isize)
            })
        })
        .collect::<FxHashSet<_>>()
}

fn run_simulation<const MAX_ITERATIONS: u64>(positions: &mut FxHashSet<Vector2>) -> u64 {
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
            let x = current_pos.x();
            let y = current_pos.y();

            let move_elf =
                (-1..=1).cartesian_product(-1..=1)
                .filter(|&(d_x, d_y)| d_x != 0 || d_y != 0)
                .any(|(d_x, d_y)| positions.contains(&Vector2::new(x + d_x, y + d_y)));

            moved_elf |= move_elf;

            let next_pos = if move_elf {
                let order =
                    search_order
                    .iter()
                    .find(|order| {
                        order
                        .iter()
                        .all(|(d_x, d_y)| {
                            !positions.contains(&Vector2::new(x + d_x, y + d_y))
                        })
                    });

                if let Some(order) = order {
                    Vector2::new(x + order[0].0, y + order[0].1)

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
    for &point in &positions {
        min_x = cmp::min(point.x(), min_x);
        min_y = cmp::min(point.y(), min_y);
        max_x = cmp::max(point.x(), max_x);
        max_y = cmp::max(point.y(), max_y);
    }

    ((max_x - min_x + 1) * (max_y - min_y + 1) - (positions.len() as isize)) as u64
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<{u64::MAX}>(&mut parse_input(input))
}