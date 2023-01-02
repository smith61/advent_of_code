fn run_simulation(input: &str, mut callback: impl FnMut(i64, i64)) {
    let mut cycle_count = 1;
    let mut x_val = 1;
    for line in input.lines() {
        let as_bytes = line.as_bytes();
        callback(cycle_count, x_val);
        cycle_count += 1;

        if as_bytes[0] == b'a' {
            let mut index = 5;
            let mut sign = 1;
            if as_bytes[index] == b'-' {
                sign = -1;
                index += 1;
            }

            let mut val = 0;
            while index < as_bytes.len() {
                val = (val * 10) + ((as_bytes[index] - b'0') as i64);
                index += 1;
            }

            callback(cycle_count, x_val);
            cycle_count += 1;

            val *= sign;
            x_val += val;
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut score = 0;
    run_simulation(
        input,
        |cycle_count, x_val| {
            if (cycle_count - 20) % 40 == 0 {
                score += cycle_count * x_val;
            }
        });

    score
}

pub fn part2(input: &str) -> String {
    let mut result = String::with_capacity(240 + 7);
    result += "\n";
    run_simulation(
        input,
        |cycle_count, x_val| {
            let col = ((cycle_count - 1) % 40) + 1;
            if col >= x_val && col < (x_val + 3) {
                result += "#";

            } else {
                result += " ";
            }

            if col == 40 {
                result += "\n";
            }
        });

    result
}