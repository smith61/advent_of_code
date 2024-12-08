
fn is_safe(levels: &[u64]) -> bool {
    if levels[0] < levels[1] {
        for i in 1..levels.len() {
            if levels[i-1] > levels[i] {
                return false;
            }

            let diff = levels[i] - levels[i-1];
            if diff < 1 || diff > 3 {
                return false;
            }
        }

    } else {
        for i in 1..levels.len() {
            if levels[i-1] < levels[i] {
                return false;
            }

            let diff = levels[i-1] - levels[i];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    true
}

pub fn part1(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        let mut parts = line.split(" ");
        let first = parts.next().unwrap().parse::<u64>().unwrap();
        let second = parts.next().unwrap().parse::<u64>().unwrap();

        count += 1;
        if first < second {
            let mut current = second;
            let mut previous = first;
            loop {
                if previous > current {
                    count -= 1;
                    break;
                }

                let diff = current - previous;
                if diff < 1 || diff > 3 {
                    count -= 1;
                    break;
                }

                if let Some(next) = parts.next() {
                    previous = current;
                    current = next.parse().unwrap();

                } else {
                    break;
                }
            }

        } else {
            let mut current = second;
            let mut previous = first;
            loop {
                if previous < current {
                    count -= 1;
                    break;
                }

                let diff = previous - current;
                if diff < 1 || diff > 3 {
                    count -= 1;
                    break;
                }

                if let Some(next) = parts.next() {
                    previous = current;
                    current = next.parse().unwrap();

                } else {
                    break;
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        let levels =
            line.split(" ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

        if is_safe(&levels) {
            count += 1;
            continue;
        }

        for i in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(i);
            if is_safe(&new_levels) {
                count += 1;
                break;
            }
        }
                                          
    }

    count
}