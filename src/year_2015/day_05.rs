
const fn to_index(b: u8) -> usize {
    (b - b'a') as usize
}

pub fn part1(input: &str) -> u64 {
    let vowel_map = b"abcdefghijklmnopqrstuvwxyz".map(|b| b"aeiou".contains(&b) as u32);
    let mut bad_map = [[0; 26]; 27];
    bad_map[to_index(b'a')][to_index(b'b')] = 1;
    bad_map[to_index(b'c')][to_index(b'd')] = 1;
    bad_map[to_index(b'p')][to_index(b'q')] = 1;
    bad_map[to_index(b'x')][to_index(b'y')] = 1;

    let mut count = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut vowel_count = 0;
        let mut double_count = 0;
        let mut bad_count = 0;

        let mut prev_index = 26;
        for &b in bytes {
            let index = to_index(b);
            vowel_count += vowel_map[index];
            double_count += (prev_index == index) as u32;
            bad_count += bad_map[prev_index][index];
            prev_index = index;
        }

        if (vowel_count >= 3) && (double_count >= 1) && (bad_count == 0) {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut has_seen = [[usize::MAX; 26]; 26];

        let mut has_double = false;
        let mut has_pair = false;
        for i in 1..bytes.len() {
            if i >= 2 {
                if bytes[i] == bytes[i - 2] {
                    has_pair = true;
                }
            }

            let prev_index = to_index(bytes[i - 1]);
            let cur_index = to_index(bytes[i - 0]);
            let last_seen = has_seen[prev_index][cur_index];
            if last_seen == usize::MAX {
                has_seen[prev_index][cur_index] = i;

            } else if last_seen <= (i - 2) {
                has_double = true;
            }
        }

        if has_double && has_pair {
            count += 1;
        }
    }

    count
}