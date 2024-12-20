use std::{collections::VecDeque, u64};

use crate::utils::{Matrix2DBorrowed, Matrix2DOwned, Vector2};


pub fn part1(input: Matrix2DBorrowed<u8>) -> u64 {
    let mut end_position = Vector2::default();
    let mut start_position = Vector2::default();
    for r in 0..input.row_count() {
        for c in 0..input.col_count() {
            if input[(c, r)] == b'E' {
                end_position = (c, r).into();
            }

            if input[(c, r)] == b'S' {
                start_position = (c, r).into();
            }
        }
    }

    let mut end_distance = Matrix2DOwned::new(input.row_count(), input.col_count());
    end_distance.backing_store_mut().fill(u64::MAX);

    let mut start_distance = Matrix2DOwned::new(input.row_count(), input.col_count());
    start_distance.backing_store_mut().fill(u64::MAX);

    {
        let mut current_cost = 0;
        let mut current_queue = VecDeque::new();
        current_queue.push_back(end_position);
        let mut next_queue = VecDeque::new();
        while !current_queue.is_empty() {
            while let Some(position) = current_queue.pop_front() {
                if end_distance[position] != u64::MAX {
                    continue;
                }

                end_distance[position] = current_cost;
                for adj in position.adjacent_points() {
                    if !input.contains(adj) {
                        continue;
                    }

                    if end_distance[adj] != u64::MAX {
                        continue;
                    }

                    if input[adj] == b'#' {
                        continue;
                    }

                    next_queue.push_back(adj);
                }
            }

            std::mem::swap(&mut current_queue, &mut next_queue);
            current_cost += 1;
        }
    }

    {
        let mut current_cost = 0;
        let mut current_queue = VecDeque::new();
        current_queue.push_back(start_position);
        let mut next_queue = VecDeque::new();
        while !current_queue.is_empty() {
            while let Some(position) = current_queue.pop_front() {
                if start_distance[position] != u64::MAX {
                    continue;
                }

                start_distance[position] = current_cost;
                for adj in position.adjacent_points() {
                    if !input.contains(adj) {
                        continue;
                    }

                    if start_distance[adj] != u64::MAX {
                        continue;
                    }

                    if input[adj] == b'#' {
                        continue;
                    }

                    next_queue.push_back(adj);
                }
            }

            std::mem::swap(&mut current_queue, &mut next_queue);
            current_cost += 1;
        }
    }

    let mut count = 0;
    let orig_distance = end_distance[start_position];
    for r in 0..input.row_count() {
        for c in 0..input.col_count() {
            let mid_point: Vector2 = (c, r).into();
            if input[mid_point] == b'#' {
                continue;
            }

            if start_distance[mid_point] == u64::MAX {
                continue;
            }

            for r_offset in -2..=2 {
                for c_offset in -2..=2 {
                    let offset_vec = Vector2::new(c_offset, r_offset);
                    if offset_vec.manhattan_distance() != 2 {
                        continue;
                    }

                    let end_point = mid_point + offset_vec;
                    if !input.contains(end_point) {
                        continue;
                    }

                    if input[end_point] == b'#' {
                        continue;
                    }

                    if end_distance[end_point] == u64::MAX {
                        continue;
                    }

                    let new_distance = start_distance[mid_point] + end_distance[end_point] + 2;
                    if (new_distance + 100) <= orig_distance {
                        count += 1;
                    }
                }
            }
        }
    }


    count
}

pub fn part2(input: Matrix2DBorrowed<u8>) -> u64 {
    let mut end_position = Vector2::default();
    let mut start_position = Vector2::default();
    for r in 0..input.row_count() {
        for c in 0..input.col_count() {
            if input[(c, r)] == b'E' {
                end_position = (c, r).into();
            }

            if input[(c, r)] == b'S' {
                start_position = (c, r).into();
            }
        }
    }

    let mut end_distance = Matrix2DOwned::new(input.row_count(), input.col_count());
    end_distance.backing_store_mut().fill(u64::MAX);

    let mut start_distance = Matrix2DOwned::new(input.row_count(), input.col_count());
    start_distance.backing_store_mut().fill(u64::MAX);

    {
        let mut current_cost = 0;
        let mut current_queue = VecDeque::new();
        current_queue.push_back(end_position);
        let mut next_queue = VecDeque::new();
        while !current_queue.is_empty() {
            while let Some(position) = current_queue.pop_front() {
                if end_distance[position] != u64::MAX {
                    continue;
                }

                end_distance[position] = current_cost;
                for adj in position.adjacent_points() {
                    if !input.contains(adj) {
                        continue;
                    }

                    if end_distance[adj] != u64::MAX {
                        continue;
                    }

                    if input[adj] == b'#' {
                        continue;
                    }

                    next_queue.push_back(adj);
                }
            }

            std::mem::swap(&mut current_queue, &mut next_queue);
            current_cost += 1;
        }
    }

    {
        let mut current_cost = 0;
        let mut current_queue = VecDeque::new();
        current_queue.push_back(start_position);
        let mut next_queue = VecDeque::new();
        while !current_queue.is_empty() {
            while let Some(position) = current_queue.pop_front() {
                if start_distance[position] != u64::MAX {
                    continue;
                }

                start_distance[position] = current_cost;
                for adj in position.adjacent_points() {
                    if !input.contains(adj) {
                        continue;
                    }

                    if start_distance[adj] != u64::MAX {
                        continue;
                    }

                    if input[adj] == b'#' {
                        continue;
                    }

                    next_queue.push_back(adj);
                }
            }

            std::mem::swap(&mut current_queue, &mut next_queue);
            current_cost += 1;
        }
    }

    let mut count = 0;
    let orig_distance = end_distance[start_position];
    for r in 0..input.row_count() {
        for c in 0..input.col_count() {
            let mid_point: Vector2 = (c, r).into();
            if input[mid_point] == b'#' {
                continue;
            }

            if start_distance[mid_point] == u64::MAX {
                continue;
            }

            for r_offset in -20..=20 {
                for c_offset in -20..=20 {
                    let offset_vec = Vector2::new(c_offset, r_offset);
                    if offset_vec.manhattan_distance() > 20 {
                        continue;
                    }

                    let end_point = mid_point + offset_vec;
                    if !input.contains(end_point) {
                        continue;
                    }

                    if input[end_point] == b'#' {
                        continue;
                    }

                    if end_distance[end_point] == u64::MAX {
                        continue;
                    }

                    let new_distance = start_distance[mid_point] + end_distance[end_point] + offset_vec.manhattan_distance() as u64;
                    if (new_distance + 100) <= orig_distance {
                        count += 1;
                    }
                }
            }
        }
    }


    count
}
