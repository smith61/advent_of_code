fn solve<const COUNT: usize>(input: &str) -> u64 {
    let as_bytes = input.as_bytes();
    let mut mask = 0u64;
    for index in 0..COUNT {
        mask ^= 1 << (as_bytes[index] - b'a');
    }

    for index in COUNT..as_bytes.len() {
        if mask.count_ones() as usize == COUNT {
            return index as u64;
        }

        mask ^= 1 << (as_bytes[index - COUNT] - b'a');
        mask ^= 1 << (as_bytes[index] - b'a');
    }

    unreachable!()
}

pub fn part1(input: &str) -> u64 {
    solve::<4>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<14>(input)
}