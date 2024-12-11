use fxhash::FxHashMap;

const fn create_split_masks() -> [(u64, Option<u64>); 20] {
    let mut split_masks = [(0, None); 20];
    split_masks[0] = (9, None);

    let mut current_mask = 10;
    let mut index = 1;
    while index < 19 {
        let mask = if (index % 2) != 0 {
            current_mask *= 10;
            Some(current_mask / 10)

        } else {
            None
        };

        split_masks[index] = (split_masks[index - 1].0 * 10 + 9, mask);
        index += 1;
    }

    split_masks[19] = (u64::MAX, Some(current_mask));
    split_masks
}

fn try_split_digits(number: u64) -> Option<(u64, u64)> {
    const SPLIT_MASKS: [(u64, Option<u64>); 20] = create_split_masks();

    let mut entry_index = 0;
    while number > SPLIT_MASKS[entry_index].0 {
        entry_index += 1;
    }

    if let (_, Some(split_mask)) = SPLIT_MASKS[entry_index] {
        let left = number / split_mask;
        let right = number % split_mask;
        Some((left, right))

    } else {
        None
    }
}

fn solve<const DEPTH: u8>(input: &str) -> u64 {
    let mut current_map = FxHashMap::default();
    let mut next_map = FxHashMap::default();

    for number in input.trim().split(" ").map(|p| p.parse::<u64>().unwrap()) {
        *current_map.entry(number).or_default() += 1;
    }

    for _ in 0..DEPTH {
        next_map.clear();
        for (&key, &value) in &current_map {
            if key == 0 {
                *next_map.entry(1).or_default() += value;

            } else if let Some((left, right)) = try_split_digits(key) {
                *next_map.entry(left).or_default() += value;
                *next_map.entry(right).or_default() += value;

            } else {
                *next_map.entry(key * 2024).or_default() += value;
            }
        }

        std::mem::swap(&mut current_map, &mut next_map);
    }

    current_map.values().sum::<u64>()
}

pub fn part1(input: &str) -> u64 {
    solve::<25>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<75>(input)
}
