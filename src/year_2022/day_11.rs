use std::collections::VecDeque;

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Clone)]
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

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    t_target: usize,
    f_target: usize,
    inpsect_count: u64
}

fn do_iteration<const DIVISOR: u64>(monkey: &mut Monkey, item: &mut u64, g_divisor: u64) -> usize {
    monkey.inpsect_count += 1;
    *item = (monkey.operation.eval(*item) / DIVISOR) % g_divisor;
    if (*item % monkey.divisor) == 0 {
        monkey.t_target

    } else {
        monkey.f_target
    }
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

    let mut seen_set = FxHashMap::<(u64, usize), usize>::with_capacity_and_hasher(ROUNDS, Default::default());
    let mut round_states = vec![0u16; ROUNDS];
    let mut inspect_counts = vec![0u16; monkeys.len()];
    for m_index in 0..monkeys.len() {
        for mut item in std::mem::take(&mut monkeys[m_index].items) {
            let mut current_position = m_index;
            let mut round = 0;
            while round < ROUNDS {
                if let Some(prev_round) = seen_set.get(&(item, current_position)) {
                    inspect_counts.fill(0);
                    for c_r in *prev_round..round {
                        for m in 0..monkeys.len() {
                            if (round_states[c_r] & (1 << m)) != 0 {
                                inspect_counts[m] += 1;
                            }
                        }
                    }

                    let cycle_rounds = round - prev_round;
                    let repeat_count = (ROUNDS - round) / cycle_rounds;
                    for m in 0..monkeys.len() {
                        monkeys[m].inpsect_count += (inspect_counts[m] as u64) * (repeat_count as u64);
                    }

                    round += cycle_rounds * repeat_count;
                    break;
                }

                seen_set.insert((item, current_position), round);
                let mut prev_position = current_position;
                let mut seen_monkeys = 0;
                loop {
                    seen_monkeys |= 1 << current_position;
                    current_position = do_iteration::<DIVISOR>(&mut monkeys[current_position], &mut item, g_divisor);
                    if current_position <= prev_position {
                        break;
                    }

                    prev_position = current_position;
                }

                round_states[round] = seen_monkeys;
                round += 1;
            }

            while round < ROUNDS {
                let mut prev_position = current_position;
                loop {
                    current_position = do_iteration::<DIVISOR>(&mut monkeys[current_position], &mut item, g_divisor);
                    if current_position <= prev_position {
                        break;
                    }

                    prev_position = current_position;
                }

                round += 1;
            }
            
            seen_set.clear();
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