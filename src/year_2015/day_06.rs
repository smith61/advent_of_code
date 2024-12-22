use crate::{scaffold::InputParser, utils::{Matrix2DOwned, Vector2}};

pub fn part1(input: &str) -> u64 {
    let mut grid = Matrix2DOwned::new(1001, 1001);
    for line in input.trim().lines() {
        let mut parser = InputParser::new(line);
        let mut points = parser.next_vector2s::<2>().unwrap();
        points[1] += Vector2::new(1, 1);

        let op = if line.starts_with("turn on") {
            0

        } else if line.starts_with("toggle") {
            1

        } else {
            assert!(line.starts_with("turn off"));

            2
        };

        for r in points[0].y()..points[1].y() {
            for c in points[0].x()..points[1].x() {
                let point = Vector2::new(c, r);
                if op == 0 {
                    grid[point] = true;

                } else if op == 1 {
                    grid[point] = !grid[point];

                } else {
                    grid[point] = false;
                }
            }
        }
    }

    grid.backing_store()
        .iter()
        .map(|v| *v as u64)
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let mut grid = Matrix2DOwned::<u16>::new(1001, 1001);
    for line in input.trim().lines() {
        let mut parser = InputParser::new(line);
        let mut points = parser.next_vector2s::<2>().unwrap();
        points[1] += Vector2::new(1, 1);

        let op = if line.starts_with("turn on") {
            0

        } else if line.starts_with("toggle") {
            1

        } else {
            assert!(line.starts_with("turn off"));

            2
        };

        for r in points[0].y()..points[1].y() {
            for c in points[0].x()..points[1].x() {
                let point = Vector2::new(c, r);
                if op == 0 {
                    grid[point] += 1;

                } else if op == 1 {
                    grid[point] += 2;

                } else {
                    if grid[point] > 0 {
                        grid[point] -= 1;
                    }
                }
            }
        }
    }

    grid.backing_store()
        .iter()
        .map(|v| *v as u64)
        .sum::<u64>()
}