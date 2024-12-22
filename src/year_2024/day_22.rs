
use crate::scaffold::InputParser;

fn generate_next_number(current_number: u64) -> u64 {
    let mut number = current_number;

    number = ((number * 64) ^ number) % 16777216;
    number = ((number / 32) ^ number) % 16777216;
    number = ((number * 2048) ^ number) % 16777216;
    number
}

pub fn part1(mut input: InputParser) -> u64 {
    let mut result = 0;
    while let Some(mut number) = input.next_uint() {
        for _ in 0..2000 {
            number = generate_next_number(number);
        }

        result += number;
    }

    result
}

pub fn part2(mut input: InputParser) -> u64 {
    const SHIFT_VALUE: usize = 20;
    const MASK_VALUE: usize = SHIFT_VALUE * SHIFT_VALUE * SHIFT_VALUE * SHIFT_VALUE;

    let mut current_max = 0;
    let mut sequence_counts = vec![0u32; MASK_VALUE];
    let mut visited = vec![0u32; MASK_VALUE];

    let mut line_number = 1;
    while let Some(mut number) = input.next_uint() {
        let mut previous_price = (number % 10) as u32;
        let mut current_sequence = 0;
        for _ in 0..3 {
            number = generate_next_number(number);
            let next_price = (number % 10) as u32;
            let diff = (next_price + 9 - previous_price) as usize;
            previous_price = next_price;
            
            current_sequence = ((current_sequence * SHIFT_VALUE) + diff) % MASK_VALUE;
        }

        for _ in 3..2000 {
            number = generate_next_number(number);
            let next_price = (number % 10) as u32;
            let diff = (next_price + 9 - previous_price) as usize;
            previous_price = next_price;

            current_sequence = ((current_sequence * SHIFT_VALUE) + diff) % MASK_VALUE;
            if visited[current_sequence] != line_number {
                assert!(visited[current_sequence] < line_number);

                visited[current_sequence] = line_number;
                sequence_counts[current_sequence] += next_price;
                current_max = current_max.max(sequence_counts[current_sequence]);
            }
        }

        line_number += 1;
    }

    current_max as u64
}
