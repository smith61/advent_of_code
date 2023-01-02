
fn solve(input: &str, lookup_table: &[u64; 16]) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let o = line.as_bytes()[0] - b'A';
        let m = line.as_bytes()[2] - b'X';
        score += lookup_table[((o << 2) | m) as usize];
    }

    score
}

pub fn part1(input: &str) -> u64 {
    const LOOKUP_TABLE: [u64; 16] = [
        1 + 3, // R-R
        2 + 6, // R-P
        3 + 0, // R-S
        0,
        1 + 0, // P-R
        2 + 3, // P-P
        3 + 6, // P-S
        0,
        1 + 6, // S-R
        2 + 0, // S-P
        3 + 3, // S-S
        0,
        0,
        0,
        0,
        0
    ];

    solve(input, &LOOKUP_TABLE)
}

pub fn part2(input: &str) -> u64 {
    const LOOKUP_TABLE: [u64; 16] = [
        3 + 0, // R-S
        1 + 3, // R-R
        2 + 6, // R-P
        0,
        1 + 0, // P-R
        2 + 3, // P-P
        3 + 6, // P-S
        0,
        2 + 0, // S-P
        3 + 3, // S-S
        1 + 6, // S-R
        0,
        0,
        0,
        0,
        0
    ];

    solve(input, &LOOKUP_TABLE)
}
