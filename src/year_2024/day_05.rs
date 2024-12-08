
pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    while let Some(rule) = lines.next() {
        if rule.is_empty() {
            break;
        }

        let mut parts = rule.split("|");
        rules.push((parts.next().unwrap().parse::<u64>().unwrap(),parts.next().unwrap().parse::<u64>().unwrap()));
    }

    let mut count = 0;
    for order in lines {
        let order =
            order.split(",")
                 .map(|v| v.parse::<u64>().unwrap())
                 .collect::<Vec<_>>();

        let mut is_valid = true;
        for (left, right) in rules.iter() {
            if let Some((l_i, _)) = order.iter().enumerate().find(|v| v.1 == left) {
                if let Some((r_i, _)) = order.iter().enumerate().find(|v| v.1 == right) {
                    if l_i > r_i {
                        is_valid = false;
                        break;
                    }
                }
            }
        }

        if is_valid {
            count += order[order.len() / 2];
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    while let Some(rule) = lines.next() {
        if rule.is_empty() {
            break;
        }

        let mut parts = rule.split("|");
        rules.push((parts.next().unwrap().parse::<u64>().unwrap(),parts.next().unwrap().parse::<u64>().unwrap()));
    }

    let mut count = 0;
    for order in lines {
        let mut order =
            order.split(",")
                 .map(|v| v.parse::<u64>().unwrap())
                 .collect::<Vec<_>>();

        let mut is_valid = false;
        let mut had_change = true;
        while had_change {
            had_change = false;
            for (left, right) in rules.iter() {
                if let Some((l_i, _)) = order.iter().enumerate().find(|v| v.1 == left) {
                    if let Some((r_i, _)) = order.iter().enumerate().find(|v| v.1 == right) {
                        if l_i > r_i {
                            is_valid = true;

                            order.insert(l_i + 1, *right);
                            order.remove(r_i);
                            had_change = true;
                        }
                    }
                }
            }
        }

        if is_valid {
            count += order[order.len() / 2];
        }
    }

    count
}