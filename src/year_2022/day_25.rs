fn from_snafu(s: &str) -> i64 {
    let mut val = 0;
    let mut position = 1;
    for c in s.chars().rev() {
        let p = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!()
        };

        val += p * position;
        position *= 5;
    }

    val
}

fn to_snafu(mut num: i64) -> String {
    if num == 0 {
        return "0".to_owned();
    }

    let mut result = "".to_owned();
    while num > 0 {
        let rem = num % 5;
        num /= 5;
        let c = match rem {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => {
                num += 1;
                "="
            },
            4 => {
                num += 1;
                "-"
            }
            _ => panic!()
        };

        result += c;
    }

    result.chars().rev().collect()
}

pub fn part1(input: &str) -> String {
    to_snafu(
        input
        .lines()
        .map(|line| from_snafu(line))
        .sum())
}

pub fn part2(_: &str) -> u64 {
    0
}
