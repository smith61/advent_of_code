
use std::collections::VecDeque;

use aoc_parse::{parser, prelude::*};

fn solve<const MG: bool>(input: &str) -> String {
    let mut stacks = [0; 9].map(|_| VecDeque::<u8>::new());
    let mut lines = input.lines();
    for line in &mut lines {
        let as_bytes = line.as_bytes();
        if as_bytes[1] == b'1' {
            break;
        }

        for (index, i) in (1..as_bytes.len()).step_by(4).enumerate() {
            if as_bytes[i] == b' ' {
                continue;
            }

            stacks[index].push_front(as_bytes[i]);
        }
    }

    lines.next();

    let parser = parser!("move " u64 " from " usize " to " usize);
    let mut copy_stack = VecDeque::new();
    for line in lines {
        let (count, from, to) = parser.parse(line).unwrap();
        if MG {
            for _ in 0..count {
                copy_stack.push_back(stacks[from - 1].pop_back().unwrap());
            }

            for _ in 0..count {
                stacks[to - 1].push_back(copy_stack.pop_back().unwrap());
            }

        } else {
            for _ in 0..count {
                let item = stacks[from - 1].pop_back().unwrap();
                stacks[to - 1].push_back(item);
            }
        }
    }

    stacks
    .into_iter()
    .map(|mut stack| stack.pop_back().unwrap() as char)
    .collect::<String>()
}

pub fn part1(input: &str) -> String {
    solve::<false>(input)
}

pub fn part2(input: &str) -> String {
    solve::<true>(input)
}