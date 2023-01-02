use itertools::Itertools;

fn get_index(b: u8) -> u64 {
    if b >= b'A' && b <= b'Z' {
        (b - b'A' + 26) as u64

    } else {
        (b - b'a' + 0) as u64
    }
}

pub fn part1(input: &str) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let as_bytes = line.as_bytes();
        let mut mask = 0u64;
        for i in 0..as_bytes.len()/2 {
            let index = get_index(as_bytes[i]);
            mask |= 1 << index;
        }

        for i in as_bytes.len()/2..as_bytes.len() {
            let index = get_index(as_bytes[i]);
            if mask & (1 << index) != 0 {
                score += index + 1;
                break;
            }
        }
    }

    score
}

pub fn part2(input: &str) -> u64 {
    let mut score = 0;
    for group in &input.lines().chunks(3) {
        let mut group_mask = u64::MAX;
        for line in group {
            let mut mask = 0u64;
            for b in line.as_bytes() {
                mask |= 1 << get_index(*b);
            }
            
            group_mask &= mask;
        }

        score += (group_mask.trailing_zeros() as u64) + 1;
    }

    score
}