
use fxhash::FxHashMap;

fn count_possibilities<'a>(towels: &[&str], pattern: &'a str, memo: &mut FxHashMap<&'a str, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }

    if memo.contains_key(pattern) {
        return *memo.get(pattern).unwrap();
    }

    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += count_possibilities(towels, &pattern[towel.len()..], memo);
        }
    }

    memo.insert(pattern, count);
    count
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let available_towels =
        lines
            .next()
            .unwrap()
            .split(",")
            .map(|p| p.trim())
            .collect::<Vec<_>>();

    lines.next().unwrap();
    let mut count = 0;

    let mut memo = FxHashMap::default();
    for line in lines {
        if count_possibilities(&available_towels, line, &mut memo) != 0 {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let available_towels =
        lines
            .next()
            .unwrap()
            .split(",")
            .map(|p| p.trim())
            .collect::<Vec<_>>();

    lines.next().unwrap();
    let mut count = 0;
    let mut memo = FxHashMap::default();
    for line in lines {
        count += count_possibilities(&available_towels, line, &mut memo);
    }

    count
}
