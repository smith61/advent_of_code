use std::{cmp::Ordering, str::FromStr};

fn parse<T: FromStr>(string: &str) -> T {
    string.parse::<T>().unwrap_or_else(|_| panic!("Failed to parse input value"))
}

fn solve<const COUNT_VALIDS: bool>(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let mut invalid_orders = [[false; 256]; 256];
    while let Some(rule) = lines.next() {
        if rule.is_empty() {
            break;
        }

        let mut parts = rule.split("|");
        let (left, right) = (parse::<usize>(parts.next().unwrap()), parse::<usize>(parts.next().unwrap()));
        invalid_orders[right][left] = true;
    }

    let mut count = 0;
    for order in lines {
        let orig_order =
            order.split(",")
                 .map(|v| parse::<usize>(v))
                 .collect::<Vec<_>>();

        let mut sorted_order = orig_order.clone();
        sorted_order.sort_unstable_by(|&left, &right| {
            if left == right {
                return Ordering::Equal;
            }

            if invalid_orders[left][right] {
                return Ordering::Greater;

            } else {
                return Ordering::Less;
            }
        });

        if COUNT_VALIDS {
            if orig_order == sorted_order {
                count += sorted_order[sorted_order.len() / 2] as u64;
            }

        } else {
            if orig_order != sorted_order {
                count += sorted_order[sorted_order.len() / 2] as u64;
            }
        }
    }

    count
}

pub fn part1(input: &str) -> u64 {
    solve::<true>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<false>(input)
}