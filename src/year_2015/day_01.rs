
pub fn part1(input: &str) -> i64 {
    let mut count = 0;
    for &b in input.trim().as_bytes() {
        if b == b'(' {
            count += 1;

        } else {
            count -= 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;
    for (index, &b) in input.trim().as_bytes().into_iter().enumerate() {
        if b == b'(' {
            count += 1;

        } else {
            count -= 1;
        }

        if count < 0 {
            return (index + 1) as u64;
        }
    }

    unreachable!()
}
