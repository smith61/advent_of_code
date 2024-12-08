use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let mut distance = 0;

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let mut parts = line.split("   ");
        let (left, right) = (parts.next().unwrap().trim().parse::<u64>().unwrap(),
                                       parts.next().unwrap().trim().parse::<u64>().unwrap());

        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        distance += if left < right {
            right - left

        } else {
            left - right
        };
    }

    distance
}

pub fn part2(input: &str) -> u64 {
    let mut similarity = 0;

    let mut left_list = Vec::new();
    let mut right_list = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split("   ");
        let (left, right) = (parts.next().unwrap().trim().parse::<u64>().unwrap(),
                                       parts.next().unwrap().trim().parse::<u64>().unwrap());

        left_list.push(left);
        right_list.entry(right).and_modify(|v| *v += 1).or_insert(1);
    }

    for left in left_list {
        if let Some(count) = right_list.get(&left) {
            similarity += left * count;
        }
    }

    similarity
}