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

fn count_stones(number: u64, current_depth: u8, seen_values: &mut FxHashMap<(u64, u8), u64>) -> u64 {
    if current_depth == 0 {
        return 1;
    }

    let key = (number, current_depth);
    if let Some(&cached_value) = seen_values.get(&key) {
        return cached_value;
    }

    let number_of_stones = if number == 0 {
        count_stones(1, current_depth - 1, seen_values)

    } else if let Some((left, right)) = try_split_digits(number) {
        count_stones(left, current_depth - 1, seen_values) + count_stones(right, current_depth - 1, seen_values)

    } else {
        count_stones(number * 2024, current_depth - 1, seen_values)
    };

    seen_values.insert(key, number_of_stones);
    number_of_stones
}

fn solve<const DEPTH: u8>(input: &str) -> u64 {
    let input = input.trim();
    
    let mut seen_values = FxHashMap::default();
    if DEPTH == 25 {
        seen_values.reserve(5000);

    } else {
        seen_values.reserve(200000);
    }

    input.trim()
         .split(" ")
         .map(|p| p.parse::<u64>().unwrap())
         .map(|number| count_stones(number, DEPTH, &mut seen_values))
         .sum::<u64>()
}

pub fn part1(input: &str) -> u64 {
    solve::<25>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<75>(input)
}
