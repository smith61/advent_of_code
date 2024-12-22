
fn generate_next_number(mut number: u32) -> u32 {
    number = ((number << 6) ^ number) & (16777216 - 1);
    number = ((number >> 5) ^ number) & (16777216 - 1);
    number = ((number << 11) ^ number) & (16777216 - 1);
    number
}

pub fn part1(input: &str) -> u64 {
    let mut nums =
        input.trim()
             .lines()
             .map(|l| l.parse::<u32>().unwrap())
             .collect::<Vec<_>>();

    for _ in 0..2000 {
        for num in &mut nums {
            *num = generate_next_number(*num);
        }
    }

    nums.into_iter().fold(0, |l, r| l + (r as u64))
}

pub fn part2(input: &str) -> u64 {
    let nums =
        input.trim()
             .lines()
             .map(|l| l.parse::<u32>().unwrap())
             .collect::<Vec<_>>();

    const SHIFT_VALUE: usize = 5;
    const MASK_VALUE: usize = (1 << (SHIFT_VALUE * 4)) - 1;

    let mut sequence_counts = vec![0u32; MASK_VALUE];
    let mut visited = vec![0u16; MASK_VALUE];

    let mut current_max = 0;
    let mut line_number = 1;
    for mut num in nums {
        let mut previous_price = num % 10;
        let mut sequence = 0;
        for _ in 0..3 {
            num = generate_next_number(num);
            let next_price = num % 10;
            let diff = (next_price + 9 - previous_price) as usize;
            previous_price = next_price;

            sequence = (sequence << SHIFT_VALUE) | diff;
        }

        for _ in 3..2000 {
            num = generate_next_number(num);
            let next_price = num % 10;
            let diff = (next_price + 9 - previous_price) as usize;
            previous_price = next_price;

            sequence = ((sequence << SHIFT_VALUE) | diff) & MASK_VALUE;
            if visited[sequence] != line_number {
                visited[sequence] = line_number;
                sequence_counts[sequence] += next_price;
                current_max = current_max.max(sequence_counts[sequence]);
            }
        }

        line_number += 1;
    }

    current_max as u64
}
