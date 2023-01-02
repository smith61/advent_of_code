
use crate::utils::Point3D;

use std::collections::VecDeque;

use fxhash::FxHashSet;

fn parse_input(input: &str) -> FxHashSet<Point3D> {
    let mut points = FxHashSet::default();
    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut index = 0;

        let x = {
            let mut val = 0;
            while bytes[index] != b',' {
                val = (val * 10) + ((bytes[index] - b'0') as isize);
                index += 1;
            }

            val
        };

        index += 1;
        let y = {
            let mut val = 0;
            while bytes[index] != b',' {
                val = (val * 10) + ((bytes[index] - b'0') as isize);
                index += 1;
            }

            val
        };

        index += 1;
        let z = {
            let mut val = 0;
            while index < bytes.len() {
                val = (val * 10) + ((bytes[index] - b'0') as isize);
                index += 1;
            }

            val
        };

        points.insert(Point3D::new(x, y, z));
    }

    points
}

pub fn part1(input: &str) -> u64 {
    let points = parse_input(input);
    let mut edges = 0;
    for point in &points {
        for n_point in point.adjacent_points() {
            if !points.contains(&n_point) {
                edges += 1;
            }
        }
    }

    edges
}

pub fn part2(input: &str) -> u64 {
    let points = parse_input(input);

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut min_z = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    let mut max_z = isize::MIN;
    for point in &points {
        min_x = std::cmp::min(min_x, point.x);
        min_y = std::cmp::min(min_y, point.y);
        min_z = std::cmp::min(min_z, point.z);
        max_x = std::cmp::max(max_x, point.x);
        max_y = std::cmp::max(max_y, point.y);
        max_z = std::cmp::max(max_z, point.z);
    }
    
    min_x -= 1;
    min_y -= 1;
    min_z -= 1;
    max_x += 1;
    max_y += 1;
    max_z += 1;

    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    let mut e_edges = 0;
    queue.push_back(Point3D::new(min_x, min_y, min_z));
    visited.insert(Point3D::new(min_x, min_y, min_z));
    while let Some(point) = queue.pop_front() {
        for n_point in point.adjacent_points() {
            if n_point.x < min_x ||
               n_point.x > max_x ||
               n_point.y < min_y ||
               n_point.y > max_y ||
               n_point.z < min_z ||
               n_point.z > max_z {

                continue;
            }

            if points.contains(&n_point) {
                e_edges += 1;
                continue;
            }

            if visited.contains(&n_point) {
                continue;
            }

            visited.insert(n_point);
            queue.push_back(n_point);
        }
    }

    e_edges
}