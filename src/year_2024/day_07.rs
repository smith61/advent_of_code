
fn parse_number(chars: &mut impl Iterator<Item = char>) -> (u64, u64) {
    let mut num = 0;
    let mut shift = 1;
    while let Some(c) = chars.next() {
        if !c.is_alphanumeric() {
            break;
        }

        num = num * 10 + (c as u64 - '0' as u64);
        shift *= 10;
    }

    (num, shift)
}

fn is_solveable<const ALLOW_CONCAT: bool>(nums: &[(u64, u64)], value: u64) -> bool {
    if nums.len() == 0 {
        return value == 0;
    }

    let (last_num, last_scale) = nums[nums.len() - 1];
    let rem_nums = &nums[0..nums.len() - 1];
    if last_num == 0 {
        return value == 0;
    }

    if value >= last_num {
        if is_solveable::<ALLOW_CONCAT>(rem_nums, value - last_num) {
            return true;
        }
    }

    if (value % last_num) == 0 {
        if is_solveable::<ALLOW_CONCAT>(rem_nums, value / last_num) {
            return true;
        }
    }

    if ALLOW_CONCAT {
        if (value % last_scale) == last_num {
            if is_solveable::<ALLOW_CONCAT>(rem_nums, value / last_scale) {
                return true;
            }
        }
    }

    false
}

pub fn part1(input: &str) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        let nums =
            line.split(|c| c == ':' || c == ' ')
                .map(|p| parse_number(&mut p.chars()))
                .collect::<Vec<_>>();

        if is_solveable::<false>(&nums[1..], nums[0].0) {
            count += nums[0].0;
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        let nums =
            line.split(|c| c == ':' || c == ' ')
                .map(|p| parse_number(&mut p.chars()))
                .collect::<Vec<_>>();

        if is_solveable::<true>(&nums[1..], nums[0].0) {
            count += nums[0].0;
        }
    }

    count
}