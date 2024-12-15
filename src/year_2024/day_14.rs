
use std::isize;

use crate::{scaffold::InputParser, utils::Point2D};

const GRID_WIDTH: isize = 101;
const GRID_HEIGHT: isize = 103;

pub fn part1(mut input: InputParser) -> u64 {
    let mut count = [0; 4];
    while let Some(ints) = input.next_ints::<4>() {
        let start_position = Point2D::new(ints[0], ints[1]);
        let vector = Point2D::new(ints[2], ints[3]);
        let end_position = {
            let mut end_position = start_position + vector * 100;
            end_position.x = end_position.x.rem_euclid(GRID_WIDTH);
            end_position.y = end_position.y.rem_euclid(GRID_HEIGHT);
            end_position
        };

        
        if end_position.x < (GRID_WIDTH / 2) {
            if end_position.y < (GRID_HEIGHT / 2) {
                count[0] += 1;

            } else if end_position.y > (GRID_HEIGHT / 2) {
                count[1] += 1;
            }

        } else if end_position.x > (GRID_WIDTH / 2) {
            if end_position.y < (GRID_HEIGHT / 2) {
                count[2] += 1;

            } else if end_position.y > (GRID_HEIGHT / 2) {
                count[3] += 1;
            }
        }
    }

    count[0] * count[1] * count[2] * count[3]
}

pub fn part2(mut input: InputParser) -> u64 {
    let mut robots = Vec::new();
    while let Some(points) = input.next_point2ds::<2>() {
        robots.push((points[0], points[1]));
    }

    let mut distance_xs_min = (isize::MAX, isize::MAX);
    let mut distance_ys_min = (isize::MAX, isize::MAX);
    for iteration in 0..(GRID_HEIGHT.max(GRID_WIDTH)) {
        let mut average_xs = 0;
        let mut average_ys = 0;
        for (position, _) in &mut robots {
            if iteration < GRID_WIDTH {
                average_xs += position.x;
            }

            if iteration < GRID_HEIGHT {
                average_ys += position.y;
            }
        }

        if iteration < GRID_WIDTH {
            average_xs /= robots.len() as isize;
        }

        if iteration < GRID_HEIGHT {
            average_ys /= robots.len() as isize;
        }

        let mut distance_xs = 0;
        let mut distance_ys = 0;
        for (position, vector) in &mut robots {
            if iteration < GRID_WIDTH {
                distance_xs += (position.x - average_xs).abs();
            }

            if iteration < GRID_HEIGHT {
                distance_ys += (position.y - average_ys).abs();
            }

            position.x = (position.x + vector.x).rem_euclid(GRID_WIDTH);
            position.y = (position.y + vector.y).rem_euclid(GRID_HEIGHT);
        }

        if iteration < GRID_WIDTH {
            if distance_xs < distance_xs_min.0 {
                distance_xs_min = (distance_xs, iteration);
            }
        }

        if iteration < GRID_HEIGHT {
            if distance_ys < distance_ys_min.0 {
                distance_ys_min = (distance_ys, iteration);
            }
        }
    }

    for scale in 0.. {
        let iteration = GRID_WIDTH * scale + distance_xs_min.1;
        if (iteration % GRID_HEIGHT) == distance_ys_min.1 {
            return iteration as u64;
        }
    }

    panic!("Did not find correct iteration.");
}
