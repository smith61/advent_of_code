
use crate::utils::Vector2;

use ndarray::Array2;

fn to_blizzard_iterator<'a>(input: &'a str) -> impl Iterator<Item = (Vector2, usize)> + 'a {
    input
    .lines()
    .enumerate()
    .flat_map(|(y, line)| {
        line
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, b)| **b != b'.' && **b != b'#')
        .map(move |(x, b)| {
            let dir = match b {
                b'>' => 0,
                b'<' => 1,
                b'v' => 2,
                b'^' => 3,
                _ => panic!()
            };

            (Vector2::new(x as isize, y as isize), dir)
        })
    })
}

fn run_simulation<const ITERATION_COUNT: usize>(input: &str) -> u64 {
    let grid_size =
        Vector2::new(
            input.lines().next().unwrap().len() as isize,
            input.lines().count() as isize);

    let x_period = grid_size.x() - 2;
    let y_period = grid_size.y() - 2;

    let mut start_position = Vector2::new(1, 0);
    let mut end_position = Vector2::new(grid_size.x() - 2, grid_size.y() - 1);

    let mut x_blizzard_pos = Array2::zeros((x_period as usize, grid_size.y_index()));
    let mut y_blizzard_pos = Array2::zeros((y_period as usize, grid_size.y_index()));
    for x_time in 0..x_blizzard_pos.nrows() {
        x_blizzard_pos[[x_time, start_position.y_index()]] = 1 << start_position.x();
        x_blizzard_pos[[x_time, end_position.y_index()]] = 1 << end_position.x();

        let edge_mask = (1 << (grid_size.x() - 1)) - 1 & !1;
        for r in 1..x_blizzard_pos.ncols() - 1 {
            x_blizzard_pos[[x_time, r]] = edge_mask;
        }
    }

    for y_time in 0..y_blizzard_pos.nrows() {
        y_blizzard_pos[[y_time, start_position.y_index()]] = 1 << start_position.x();
        y_blizzard_pos[[y_time, end_position.y_index()]] = 1 << end_position.x();
        let edge_mask = (1 << (grid_size.x() - 1)) - 1 & !1;
        for r in 1..y_blizzard_pos.ncols() - 1 {
            y_blizzard_pos[[y_time, r]] = edge_mask;
        }
    }

    let mut blizzard_masks = Array2::<u128>::zeros((4, grid_size.y_index()) );
    for (point, dir) in to_blizzard_iterator(input) {
        blizzard_masks[[dir, point.y_index()]] |= 1 << point.x();
    }

    for time in 0..x_blizzard_pos.nrows() {
        let left_mask = 1 << 1;
        let right_mask = 1 << x_period;

        let mut blizzard_pos = x_blizzard_pos.row_mut(time);

        for dir in 0..=1 {
            let mut masks = blizzard_masks.row_mut(dir);

            for y_index in 1..masks.len() - 1 {
                blizzard_pos[y_index] &= !masks[y_index];
                if dir == 0 {
                    let mut new_mask = masks[y_index];
                    new_mask &= !right_mask;
                    new_mask <<= 1;
                    if (masks[y_index] & right_mask) != 0 {
                        new_mask |= left_mask;
                    }

                    masks[y_index] = new_mask;

                } else {
                    let mut new_mask = masks[y_index];
                    new_mask &= !left_mask;
                    new_mask >>= 1;
                    if (masks[y_index] & left_mask) != 0 {
                        new_mask |= right_mask;
                    }

                    masks[y_index] = new_mask;
                }
            }
        }
    }
    
    for time in 0..y_blizzard_pos.nrows() {
        let mut blizzard_pos = y_blizzard_pos.row_mut(time);
        for dir in 2..=3 {
            let sign = if dir == 2 { -1 } else { 1 };
            let masks = blizzard_masks.row(dir);
            for y_index in 1..blizzard_pos.len() - 1 {
                let source_index = ((y_index as isize) - 1 + ((time as isize) * sign)).rem_euclid(y_period) + 1;
                blizzard_pos[y_index] &= !masks[source_index as usize];
            }
        }
    }

    let mut minimum_steps = 0;
    let mut x_index = 0;
    let mut y_index = 0;
    for _ in 1..=ITERATION_COUNT {
        let mut current_map = vec![0u128; grid_size.y_index()];
        let mut next_map = vec![0u128; grid_size.y_index()];
        current_map[start_position.y_index()] = 1 << start_position.x();
        loop {
            if current_map[end_position.y_index()] & (1 << end_position.x()) != 0 {
                break;
            }

            //
            // Remove all positions from the current map that are invalid.
            //

            for r in 0..grid_size.y_index() {
                current_map[r] &= x_blizzard_pos[[x_index as usize, r]];
                current_map[r] &= y_blizzard_pos[[y_index as usize, r]];
            }

            //
            // Shuffle the rows of the map up/down.
            //

            for r in 0..(grid_size.y_index() - 1) {
                next_map[r + 1] |= current_map[r];
            }

            for r in 1..grid_size.y_index() {
                next_map[r - 1] |= current_map[r];
            }

            //
            // Shuffle each row left, right, and no move.
            //

            for r in 0..grid_size.y_index() {
                next_map[r] |= current_map[r];
                next_map[r] |= current_map[r] << 1;
                next_map[r] |= current_map[r] >> 1;
            }

            std::mem::swap(&mut current_map, &mut next_map);
            next_map.fill(0);

            minimum_steps += 1;
            x_index = (x_index + 1) % x_period;
            y_index = (y_index + 1) % y_period;
        }

        std::mem::swap(&mut start_position, &mut end_position);
    }

    minimum_steps
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<1>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<3>(input)
}