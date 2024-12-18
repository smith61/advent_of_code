use std::collections::HashMap;

use crate::utils::{Matrix2DBorrowed, Matrix2DOwned, Vector2};

pub fn part1(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut antinodes = Matrix2DOwned::<bool>::new(grid.row_count(), grid.col_count());

    let mut antennas = HashMap::<u8, Vec<Vector2>>::new();
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let pos = Vector2::new(c as isize, r as isize);
            if grid[pos] != b'.' {
                antennas.entry(grid[pos]).or_default().push(pos);
            }
        }
    }

    for (_, poses) in antennas {
        for i in 0..poses.len() {
            for j in (i+1)..poses.len() {
                let pos1 = poses[i];
                let pos2 = poses[j];

                let distance = pos1 - pos2;
                let a1 = pos1 + distance;
                let a2 = pos2 - distance;
                if antinodes.contains(a1) {
                    antinodes[a1] = true;
                }

                if antinodes.contains(a2) {
                    antinodes[a2] = true;
                }
            }
        }
    }

    antinodes.backing_store()
             .iter()
             .map(|v| *v as u64)
             .sum()
}

pub fn part2(grid: Matrix2DBorrowed<u8>) -> u64 {
    let mut antinodes = Matrix2DOwned::<bool>::new(grid.row_count(), grid.col_count());

    let mut antennas = HashMap::<u8, Vec<Vector2>>::new();
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let pos = Vector2::new(c as isize, r as isize);
            if grid[pos] != b'.' {
                antennas.entry(grid[pos]).or_default().push(pos);
            }
        }
    }

    for (_, poses) in antennas {
        for i in 0..poses.len() {
            for j in (i+1)..poses.len() {
                let pos1 = poses[i];
                let pos2 = poses[j];

                let distance = (pos1 - pos2).normalize();
                let mut current_pos = pos1;
                loop {
                    if !antinodes.contains(current_pos) {
                        break;
                    }

                    antinodes[current_pos] = true;
                    current_pos += distance;
                }

                current_pos = pos2;
                loop {
                    if !antinodes.contains(current_pos) {
                        break;
                    }

                    antinodes[current_pos] = true;
                    current_pos -= distance;
                }
            }
        }
    }

    antinodes.backing_store()
             .iter()
             .map(|v| *v as u64)
             .sum()
}
