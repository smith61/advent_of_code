
pub fn part1(input: &str) -> u64 {
    let mut keys = Vec::with_capacity(300);
    let mut locks = Vec::with_capacity(300);

    for grid in input.split("\n\n") {
        let mut value = 0;
        for c in grid.chars() {
            if c == '#' {
                value = (value << 1) | 1;

            } else if c == '.' {
                value <<= 1;
            }
        }

        if (value & 1) == 1 {
            keys.push(value);

        } else {
            locks.push(value);
        }
    }


    let mut count = 0;
    for key in keys {
        for &lock in &locks {
            if (key & lock) == 0 {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(_input: &str) -> u64 {
    0
}
