use aoc_parse::{parser, prelude::*};

pub fn part1(input: &str) -> u64 {
    let mut score = 0;
    let parser = parser!(u64 "-" u64 "," u64 "-" u64);
    for line in input.lines() {
        let (s1, e1, s2, e2) = parser.parse(line).unwrap();
        if ((s1 <= s2) && (e1 >= e2)) || ((s2 <= s1) && (e2 >= e1)) {
            score += 1;
        }
    }

    score
}

pub fn part2(input: &str) -> u64 {
    let mut score = 0;
    let parser = parser!(u64 "-" u64 "," u64 "-" u64);
    for line in input.lines() {
        let (s1, e1, s2, e2) = parser.parse(line).unwrap();
        if s1 <= e2 && e1 >= s2 {
            score += 1;
        }
    }

    score
}