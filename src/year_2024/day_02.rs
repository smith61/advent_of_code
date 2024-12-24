
fn is_safe(mut levels: impl Iterator<Item = u64>) -> bool {
    let first = levels.next().unwrap();
    let second = levels.next().unwrap();

    if first < second {
        let mut current = second;
        let mut previous = first;
        loop {
            if previous > current {
                return false;
            }

            let diff = current - previous;
            if diff < 1 || diff > 3 {
                return false;
            }

            if let Some(next) = levels.next() {
                previous = current;
                current = next;

            } else {
                break;
            }
        }

    } else {
        let mut current = second;
        let mut previous = first;
        loop {
            if previous < current {
                return false;
            }

            let diff = previous - current;
            if diff < 1 || diff > 3 {
                return false;
            }

            if let Some(next) = levels.next() {
                previous = current;
                current = next;

            } else {
                break;
            }
        }
    }

    true
}

pub fn part1(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        let levels =
            line.split(" ")
                .map(|p| p.parse::<u64>().unwrap());

        if is_safe(levels) {
            count += 1;
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

        if is_safe(levels.iter().cloned()) {
            count += 1;
            continue;
        }

        for i in 0..levels.len() {
            let iter =
                levels.iter()
                      .enumerate()
                      .filter(|&(index, _)| index != i)
                      .map(|(_, l)| *l);

            if is_safe(iter) {
                count += 1;
                break;
            }
        }
                                          
    }

    count
}