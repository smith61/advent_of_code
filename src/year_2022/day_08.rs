fn to_grid(input: &str) -> Vec<&[u8]> {
    input
    .lines()
    .map(|line| line.as_bytes())
    .collect()
}

pub fn part1(input: &str) -> u64 {
    let grid = to_grid(input);
    let mut visible_set = vec![false; grid.len() * grid[0].len()];
    let to_index = |row: usize, col: usize| { row * grid[0].len() + col };

    let mut vert_heights = vec![0; grid[0].len()];
    for row_index in 0..grid.len() {
        let row = grid[row_index];
        let mut row_height = 0;
        for col_index in 0..row.len() {
            let height = row[col_index] - b'0' + 1;
            let mut visible = false;
            if height > row_height {
                visible = true;
                row_height = height;
            }

            if height > vert_heights[col_index] {
                visible = true;
                vert_heights[col_index] = height;
            }

            if visible {
                visible_set[to_index(row_index, col_index)] = true;
            }
        }
    }

    vert_heights.fill(0);
    for row_index in (0..grid.len()).rev() {
        let row = grid[row_index];
        let mut row_height = 0;
        for col_index in (0..row.len()).rev() {
            let height = row[col_index] - b'0' + 1;
            let mut visible = false;
            if height > row_height {
                visible = true;
                row_height = height;
            }

            if height > vert_heights[col_index] {
                visible = true;
                vert_heights[col_index] = height;
            }

            if visible {
                visible_set[to_index(row_index, col_index)] = true;
            }
        }
    }

    visible_set.into_iter().map(|v| v as u64).sum()
}

pub fn part2(input: &str) -> u64 {
    let grid = to_grid(input);
    
    let mut max_score = 0;
    for row_index in 0..grid.len() {
        let row = grid[row_index];
        for col_index in 0..row.len() {
            let height = row[col_index];
            let mut score = 1;
            {
                let mut visible_count = 0;
                for t_row_index in (0..row_index).rev() {
                    visible_count += 1;
                    if height <= grid[t_row_index][col_index] {
                        break;
                    }
                }

                score *= visible_count;
            }

            {
                let mut visible_count = 0;
                for t_row_index in row_index+1..grid.len() {
                    visible_count += 1;
                    if height <= grid[t_row_index][col_index] {
                        break;
                    }
                }

                score *= visible_count;
            }

            {
                let mut visible_count = 0;
                for t_col_index in (0..col_index).rev() {
                    visible_count += 1;
                    if height <= grid[row_index][t_col_index] {
                        break;
                    }
                }

                score *= visible_count;
            }

            {
                let mut visible_count = 0;
                for t_col_index in col_index+1..row.len() {
                    visible_count += 1;
                    if height <= grid[row_index][t_col_index] {
                        break;
                    }
                }

                score *= visible_count;
            }

            max_score = std::cmp::max(max_score, score);
        }
    }

    max_score
}