
fn solve<const COUNT: usize>(input: &str) -> u64 {
    let mut solution = [0; COUNT];
    let mut current_count = 0;
    for line in input.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            for i in 0..COUNT {
                if current_count > solution[i] {
                    std::mem::swap(&mut current_count, &mut solution[i]);
                }
            }

            current_count = 0;

        } else {
            current_count += line.parse::<u64>().unwrap();
        }
    }

    solution.iter().sum()
}

pub fn part1(input: &str) -> u64 {
    solve::<1>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<3>(input)
}