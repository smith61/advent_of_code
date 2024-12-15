use crate::utils::{Grid, Grid2D, Grid2DBorrowed, Point2D};

const LEFT: Point2D = Point2D::new(-1, 0);
const RIGHT: Point2D = Point2D::new(1, 0);
const UP: Point2D = Point2D::new(0, -1);
const DOWN: Point2D = Point2D::new(0, 1);

fn part1_try_and_commit_move(grid: &mut Grid2D<u8>, robot_pos: Point2D, direction: Point2D) -> Point2D {
    let next_position = robot_pos + direction;
    if grid[next_position] == b'#' {
        return robot_pos;
    }

    if grid[next_position] == b'O' {
        let mut check_pos = next_position + direction;
        loop {
            if grid[check_pos] == b'#' {
                return robot_pos;
            }

            if grid[check_pos] != b'O' {
                break;
            }

            check_pos += direction;
        }

        grid[check_pos] = b'O';
        grid[next_position] = b'.';
    }

    next_position
}

pub fn part1(input: &str) -> u64 {

    let end_grid_offset = input.find("\r\n\r\n").or_else(|| input.find("\n\n")).unwrap();

    let mut grid = Grid2DBorrowed::from_input_lines(&input[..end_grid_offset]).to_owned();
    let instructions = input[end_grid_offset..].trim().as_bytes();

    let mut robot_pos = Point2D::default();
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let point = Point2D::new(c as isize, r as isize);
            if grid[point] == b'@' {
                robot_pos = point;
            }
        }
    }

    grid[robot_pos] = b'.';

    let mut index = 0;
    while index < instructions.len() {
        let direction = match instructions[index] {
            b'<' => LEFT,
            b'>' => RIGHT,
            b'v' => DOWN,
            b'^' => UP,
            _ => {
                index += 1;
                continue;
            }
        };

        robot_pos = part1_try_and_commit_move(&mut grid, robot_pos, direction);
        index += 1;
    }

    let mut score = 0;
    for r in 1..(grid.row_count() - 1) {
        for c in 1..(grid.col_count() - 1) {
            let point = Point2D::new(c as isize, r as isize);
            if grid[point] == b'O' {
                score += (100 * r + c) as u64;
            }
        }
    }
    score
}

fn part2_can_move(grid: &Grid2D<u8>, box_position: Point2D, direction: Point2D) -> bool {
    assert_eq!(grid[box_position], b'[');

    let next_position = box_position + direction;
    let can_move = if direction == LEFT {
        if grid[next_position] == b'#' {
            false

        } else if grid[next_position] == b'.' {
            true

        } else {
            assert_eq!(grid[next_position], b']');

            part2_can_move(grid, next_position + LEFT, direction)
        }

    } else if direction == RIGHT {
        let next_position = next_position + RIGHT;
        if grid[next_position] == b'#' {
            false

        } else if grid[next_position] == b'.' {
            true

        } else {
            assert_eq!(grid[next_position], b'[');

            part2_can_move(grid, next_position, direction)
        }

    } else {
        match (grid[next_position], grid[next_position + RIGHT]) {
            (b'#', _) | (_, b'#') => false,
            (b'.', b'.') => true,
            (b'[', b']') => part2_can_move(grid, next_position, direction),
            (b'.', b'[') => part2_can_move(grid, next_position + RIGHT, direction),
            (b']', b'.') => part2_can_move(grid, next_position + LEFT, direction),
            (b']', b'[') => part2_can_move(grid, next_position + LEFT, direction) && part2_can_move(grid, next_position + RIGHT, direction),
            (l, r) => panic!("Invalid grid characters '{}'/'{}'", l as char, r as char)
        }
    };

    can_move
}

fn part2_commit_move(grid: &mut Grid2D<u8>, box_position: Point2D, direction: Point2D) {
    assert_eq!(grid[box_position], b'[');

    let next_position = box_position + direction;
    if direction == LEFT {
        if grid[next_position] == b']' {
            part2_commit_move(grid, next_position + LEFT, direction);

        } else {
            assert_eq!(grid[next_position], b'.');
        }

    } else if direction == RIGHT {
        let next_box_position = next_position + RIGHT;
        if grid[next_box_position] == b'[' {
            part2_commit_move(grid, next_box_position, direction);

        } else {
            assert_eq!(grid[next_box_position], b'.');
        }

    } else {
        if grid[next_position] == b'[' {
            part2_commit_move(grid, next_position, direction);

        } else {
            if grid[next_position] == b']' {
                part2_commit_move(grid, next_position + LEFT, direction);

            } else {
                assert_eq!(grid[next_position], b'.');
            }

            if grid[next_position + RIGHT] == b'[' {
                part2_commit_move(grid, next_position + RIGHT, direction);

            } else {
                assert_eq!(grid[next_position + RIGHT], b'.');
            }
        }
    }

    grid[box_position] = b'.';
    grid[box_position + RIGHT] = b'.';
    grid[next_position] = b'[';
    grid[next_position + RIGHT] = b']';
}

pub fn part2(input: &str) -> u64 {
    let end_grid_offset = input.find("\r\n\r\n").or_else(|| input.find("\n\n")).unwrap();

    let starting_grid = Grid2DBorrowed::from_input_lines(&input[..end_grid_offset]);
    let mut grid = Grid2D::new(starting_grid.row_count(), starting_grid.col_count() * 2);
    let mut robot_pos = Point2D::default();
    for r in 0..starting_grid.row_count() {
        for c in 0..starting_grid.col_count() {
            let starting_grid_pos = Point2D::new(c as isize, r as isize);
            let new_grid_pos = Point2D::new((c * 2) as isize, r as isize);
            let replace = match starting_grid[starting_grid_pos] {
                b'#' => (b'#', b'#'),
                b'.' => (b'.', b'.'),
                b'O' => (b'[', b']'),
                b'@' => {
                    robot_pos = new_grid_pos;
                    (b'.', b'.')
                },
                c => panic!("Unrecognied character '{}'", c as char)
            };
            grid[new_grid_pos] = replace.0;
            grid[new_grid_pos + RIGHT] = replace.1;
        }
    }

    let instructions = input[end_grid_offset..].trim().as_bytes();
    let mut index = 0;
    while index < instructions.len() {
        let direction = match instructions[index] {
            b'<' => LEFT,
            b'>' => RIGHT,
            b'v' => DOWN,
            b'^' => UP,
            _ => {
                index += 1;
                continue;
            }
        };

        index += 1;

        let next_position = robot_pos + direction;
        if grid[next_position] == b'.' {
            robot_pos = next_position;

        } else if grid[next_position] != b'#' {
            let box_position = if grid[next_position] == b'[' {
                next_position

            } else {
                next_position + LEFT
            };

            if part2_can_move(&grid, box_position, direction) {
                part2_commit_move(&mut grid, box_position, direction);
                robot_pos = next_position;
            }
        }
    }
    

    let mut score = 0;
    for r in 1..(grid.row_count() - 1) {
        for c in 1..(grid.col_count() - 1) {
            let point = Point2D::new(c as isize, r as isize);
            if grid[point] == b'[' {
                score += (100 * r + c) as u64;
            }
        }
    }

    score
}
