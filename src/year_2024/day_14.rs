
use crate::{scaffold::InputParser, utils::{Grid, Grid2D, Grid2DBorrowed, Point2D}};

const GRID_WIDTH: isize = 101;
const GRID_HEIGHT: isize = 103;

const MATCHING_GRID_INPUT: &'static str = 
"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
X.............................X
X.............................X
X.............................X
X.............................X
X..............X..............X
X.............XXX.............X
X............XXXXX............X
X...........XXXXXXX...........X
X..........XXXXXXXXX..........X
X............XXXXX............X
X...........XXXXXXX...........X
X..........XXXXXXXXX..........X
X.........XXXXXXXXXXX.........X
X........XXXXXXXXXXXXX........X
X..........XXXXXXXXX..........X
X.........XXXXXXXXXXX.........X
X........XXXXXXXXXXXXX........X
X.......XXXXXXXXXXXXXXX.......X
X......XXXXXXXXXXXXXXXXX......X
X........XXXXXXXXXXXXX........X
X.......XXXXXXXXXXXXXXX.......X
X......XXXXXXXXXXXXXXXXX......X
X.....XXXXXXXXXXXXXXXXXXX.....X
X....XXXXXXXXXXXXXXXXXXXXX....X
X.............XXX.............X
X.............XXX.............X
X.............XXX.............X
X.............................X
X.............................X
X.............................X
X.............................X
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

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

fn contains_tree(grid: &Grid2D<bool>) -> bool {
    let matching_grid = Grid2DBorrowed::from_input_lines(MATCHING_GRID_INPUT);
    for r_g in 0..(grid.row_count() - matching_grid.row_count()) {
        for c_g in 0..(grid.col_count() - matching_grid.col_count()) {

            let mut found_match = true;
            'outer: for r in 0..matching_grid.row_count() {
                for c in 0..matching_grid.col_count() {
                    let mg_point = Point2D::new(c as isize, r as isize);
                    let grid_point = Point2D::new((c_g + c) as isize, (r_g + r) as isize);
                    found_match = (matching_grid[mg_point] == b'X') == grid[grid_point];
                    if !found_match {
                        break 'outer;
                    }
                }
            }

            if found_match {
                return true;
            }
        }
    }

    false
}

pub fn part2(mut input: InputParser) -> u64 {
    let mut robots = Vec::new();
    while let Some(ints) = input.next_ints::<4>() {
        robots.push((Point2D::new(ints[0], ints[1]), Point2D::new(ints[2], ints[3])));
    }

    let mut grid = Grid2D::new(GRID_HEIGHT as usize, GRID_WIDTH as usize);
    for i in 1.. {
        for &(position, _) in &robots {
            grid[position] = false;
        }
        
        let mut has_duplicates = false;
        for (position, vector) in &mut robots {
            position.x = (position.x + vector.x).rem_euclid(GRID_WIDTH);
            position.y = (position.y + vector.y).rem_euclid(GRID_HEIGHT);
            if grid[*position] {
                has_duplicates = true;

            } else {
                grid[*position] = true;
            }
        }

        if !has_duplicates && contains_tree(&grid) {
            return i;
        }
    }

    unreachable!();
}
