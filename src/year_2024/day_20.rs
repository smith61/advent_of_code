use std::u64;

use crate::utils::{Matrix2DBorrowed, Matrix2DOwned, Vector2};

fn solve<const MAX_CHEAT_DISTANCE: isize>(input: Matrix2DBorrowed<u8>) -> u64 {
    let bottom_right: Vector2 = (input.col_count(), input.row_count()).into();
    
    let mut end_position = Vector2::default();
    let mut start_position = Vector2::default();
    for cell in input.cell_iter() {
        if input[cell] == b'E' {
            end_position = cell;
        }

        if input[cell] == b'S' {
            start_position = cell;
        }
    }

    let mut end_distance = Matrix2DOwned::new(input.row_count(), input.col_count());
    end_distance.backing_store_mut().fill(u32::MAX / 2);

    {
        let mut current_position = end_position;
        let mut previous_position = Vector2::new(-1, -1);
        let mut current_cost = 0;
        loop {
            end_distance[current_position] = current_cost;
            if current_position == start_position {
                break;
            }

            let mut next_position = current_position;
            for adj in current_position.adjacent_points() {
                if adj != previous_position &&
                   input[adj] != b'#' {

                    next_position = adj;
                    break;
                }
            }

            assert_ne!(current_position, next_position);
            previous_position = current_position;
            current_position = next_position;
            current_cost += 1;
        }
    }

    let orig_distance = end_distance[start_position];
    let threshold_distance = orig_distance - 100;

    let mut cheat_path_count = 0;

    {
        let mut current_position = start_position;
        let mut previous_position = Vector2::new(-1, -1);
        let mut current_cost = 0;
        loop {
            if end_distance[current_position] < 100 {
                break;
            }

            let bottom_right_distance = bottom_right - current_position;

            let left_max = MAX_CHEAT_DISTANCE.min(current_position.x());
            let right_max = (MAX_CHEAT_DISTANCE + 1).min(bottom_right_distance.x());
            let up_max = MAX_CHEAT_DISTANCE.min(current_position.y());
            let down_max = (MAX_CHEAT_DISTANCE + 1).min(bottom_right_distance.y());

            for r_offset in -up_max..down_max {
                let c_offset_max = MAX_CHEAT_DISTANCE - r_offset.abs();
                for c_offset in -(c_offset_max.min(left_max))..((c_offset_max + 1).min(right_max)) {
                    let offset_vec = Vector2::new(c_offset, r_offset);
                    let mh_distance = offset_vec.manhattan_distance() as u32;
                    if mh_distance < 2 {
                        continue;
                    }

                    let end_point = current_position + offset_vec;
                    let new_distance = current_cost + end_distance[end_point] + mh_distance;
                    if new_distance <= threshold_distance {
                        cheat_path_count += 1;
                    }
                }
            }

            let mut next_position = current_position;
            for adj in current_position.adjacent_points() {
                if adj != previous_position &&
                   input[adj] != b'#' {

                    next_position = adj;
                    break;
                }
            }

            assert_ne!(current_position, next_position);
            previous_position = current_position;
            current_position = next_position;
            current_cost += 1;
        }
    }

    cheat_path_count
}

pub fn part1(input: Matrix2DBorrowed<u8>) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: Matrix2DBorrowed<u8>) -> u64 {
    solve::<20>(input)
}
