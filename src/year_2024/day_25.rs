use itertools::Itertools;


pub fn part1(input: &str) -> u64 {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let mut grid_height = 0;
    let mut grid_width = 0;
    for line in input.trim().lines() {
        if line.is_empty() {
            break;
        }

        grid_height += 1;
        grid_width = line.len();
    }

    let mut pin_heights = vec![0; grid_width];
    let mut is_first_line = true;
    let mut is_lock = false;
    for line in input.trim().lines() {
        if is_first_line {
            is_lock = line == "#".repeat(grid_width);
            is_first_line = false;
        }

        if line.is_empty() {
            is_first_line = true;
            if is_lock {
                locks.push(pin_heights.clone());
            
            } else {
                keys.push(pin_heights.clone());
            }

            pin_heights.fill(0);
            continue;
        }

        for (index, c) in line.chars().enumerate() {
            if c == '#' {
                pin_heights[index] += 1;
            }
        }
    }

    if is_lock {
        locks.push(pin_heights.clone());
    
    } else {
        keys.push(pin_heights.clone());
    }

    let mut count = 0;
    for (key, lock) in keys.iter().cartesian_product(locks.iter()) {
        let mut is_valid = true;
        for i in 0..grid_width {
            if (key[i] + lock[i]) > grid_height {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            count += 1;
        }
    }

    count
}

pub fn part2(_input: &str) -> u64 {
    0
}
