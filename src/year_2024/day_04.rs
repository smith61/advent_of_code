use std::collections::VecDeque;


fn count_matches(grid: &[&[u8]], r: usize, c: usize, val: &[u8]) -> u64 {
    let mut search_list = VecDeque::new();
    for r_d in -1isize..=1 {
        for r_c in -1isize..=1 {
            if r_d == 0 && r_c == 0 {
                continue;
            }

            search_list.push_back((r, c, 0, r_d, r_c));
        }
    }

    let mut count = 0;
    while let Some((r, c, index, r_d, c_d)) = search_list.pop_front() {
        if grid[r][c] != val[index] {
            continue;
        }

        if index == (val.len() - 1) {
            count += 1;
            continue;
        }

        let r_n = (r as isize) + r_d;
        let c_n = (c as isize) + c_d;
        if r_n < 0 || r_n as usize >= grid.len() {
            continue;
        }

        if c_n < 0 || c_n as usize >= grid[0].len() {
            continue;
        }

        search_list.push_back((r_n as usize, c_n as usize, index + 1, r_d, c_d));
    }

    count
}

pub fn part1(input: &str) -> u64 {
    let mut count = 0;
    let grid =
        input.lines()
             .map(|l| l.as_bytes())
             .collect::<Vec<_>>();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            count += count_matches(&grid, r, c, b"XMAS");
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;
    let grid =
        input.lines()
             .map(|l| l.as_bytes())
             .collect::<Vec<_>>();

    for r in 1..grid.len()-1 {
        for c in 1..grid[r].len()-1 {
            if grid[r][c] != b'A' {
                continue;
            }

            let mut is_valid = false;
            if grid[r-1][c-1] == b'M' &&
               grid[r+1][c+1] == b'S' {

                is_valid = true;

            } else if grid[r-1][c-1] == b'S' &&
                      grid[r+1][c+1] == b'M' {

                is_valid = true;
            }

            if !is_valid {
                continue;
            }

            is_valid = false;
            if grid[r-1][c+1] == b'M' &&
               grid[r+1][c-1] == b'S' {
                
                is_valid = true;

            } else if grid[r-1][c+1] == b'S' &&
                      grid[r+1][c-1] == b'M' {
                
                is_valid = true;
            }

            if is_valid {
                count += 1;
            }
        }
    }

    count
}