use std::collections::VecDeque;

use itertools::Itertools;

enum Operation {
    Add(u64),
    Mul(u64),
    Square
}

impl Operation {

    fn parse_operation(op: &str) -> Operation {
        if op == "old * old" {
            Operation::Square

        } else if op.starts_with("old + ") {
            Operation::Add(u64::from_str_radix(&op["old + ".len()..], 10).unwrap())

        } else if op.starts_with("old * ") {
            Operation::Mul(u64::from_str_radix(&op["old * ".len()..], 10).unwrap())

        } else {
            unreachable!()
        }
    }

    fn eval(&self, old_val: u64) -> u64 {
        match self {
            Operation::Add(val) => old_val + val,
            Operation::Mul(val) => old_val * val,
            Operation::Square => old_val * old_val
        }
    }

}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    t_target: usize,
    f_target: usize,
    inpsect_count: u64
}

fn run_simulation<const ROUNDS: usize, const DIVISOR: u64>(input: &str) -> u64 {
    let mut monkeys = Vec::new();
    let mut g_divisor = 1;
    for mut group in &input.lines().chunks(7) {
        group.next().unwrap();

        let items =
            group
            .next()
            .map(|line| {
                let line = &line["  Starting items: ".len()..];
                line
                .split(",")
                .map(|item| u64::from_str_radix(item.trim(), 10).unwrap())
                .collect::<VecDeque<_>>()
            })
            .unwrap();

        let operation =
            group
            .next()
            .map(|line|{
                Operation::parse_operation(&line["  Operation: new = ".len()..])
            })
            .unwrap();

        let divisor =
            group
            .next()
            .map(|line| &line["  Test: divisible by ".len()..])
            .map(|val| u64::from_str_radix(val, 10).unwrap())
            .unwrap();

        let t_target =
            group
            .next()
            .map(|line| &line["    If true: throw to monkey ".len()..])
            .map(|val| usize::from_str_radix(val, 10).unwrap())
            .unwrap();

        let f_target =
            group
            .next()
            .map(|line| &line["    If false: throw to monkey ".len()..])
            .map(|val| usize::from_str_radix(val, 10).unwrap())
            .unwrap();

        g_divisor *= divisor;
        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            t_target,
            f_target,
            inpsect_count: 0
        });
    }

    let mut spare_vec = VecDeque::new();
    for _ in 0..ROUNDS {
        for index in 0..monkeys.len() {
            std::mem::swap(&mut spare_vec, &mut monkeys[index].items);
            monkeys[index].inpsect_count += spare_vec.len() as u64;
            for item in spare_vec.drain(..) {
                let mut new_val = monkeys[index].operation.eval(item);
                new_val /= DIVISOR;
                new_val %= g_divisor;
                let t_index = if new_val % monkeys[index].divisor == 0 {
                    monkeys[index].t_target

                } else {
                    monkeys[index].f_target
                };

                monkeys[t_index].items.push_back(new_val);
            }
        }
    }

    monkeys.sort_by(|l, r| r.inpsect_count.cmp(&l.inpsect_count));
    monkeys[0].inpsect_count * monkeys[1].inpsect_count
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<20, 3>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<10000, 1>(input)
}