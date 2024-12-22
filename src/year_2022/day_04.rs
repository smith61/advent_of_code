
use crate::scaffold::InputParser;

pub fn part1(mut input: InputParser) -> u64 {
    let mut score = 0;
    while let Some(nums) = input.next_uints::<4>() {
        let (s1, e1, s2, e2) = (nums[0], nums[1], nums[2], nums[3]);
        if ((s1 <= s2) && (e1 >= e2)) || ((s2 <= s1) && (e2 >= e1)) {
            score += 1;
        }
    }

    score
}

pub fn part2(mut input: InputParser) -> u64 {
    let mut score = 0;
    while let Some(nums) = input.next_uints::<4>() {
        let (s1, e1, s2, e2) = (nums[0], nums[1], nums[2], nums[3]);
        if s1 <= e2 && e1 >= s2 {
            score += 1;
        }
    }

    score
}