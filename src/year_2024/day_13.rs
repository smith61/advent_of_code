use std::{cmp::Reverse, collections::BinaryHeap, fmt::Binary, iter::Rev};

use fxhash::FxHashSet;
use z3::ast::Ast;

use crate::utils::Point2D;

fn parse_int(chars: &mut impl Iterator<Item = char>) -> isize {
    let mut value = 0;
    while let Some(c) = chars.next() {
        if !c.is_alphanumeric() {
            break;
        }

        value = (value * 10) + ((c as isize) - '0' as isize);
    }

    value
}

fn find_min_cost(destination: Point2D, button_a: Point2D, button_b: Point2D) -> u64 {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, Point2D::new(0, 0))));

    let mut visited = FxHashSet::default();
    while let Some(Reverse((cost, current_pos))) = queue.pop() {
        if current_pos == destination {
            return cost;
        }

        if current_pos.x_index() > destination.x_index() ||
           current_pos.y_index() > destination.y_index() {
            
            continue;
        }

        if visited.contains(&current_pos) {
            continue;
        }

        visited.insert(current_pos);
        queue.push(Reverse((cost + 3, current_pos + button_a)));
        queue.push(Reverse((cost + 1, current_pos + button_b)));
    }

    0
}

use crate::utils::z3::*;
use z3::{Config, Context, Optimize, SatResult, ast::Int};

fn find_min_cost_z3(destination: Point2D, button_a: Point2D, button_b: Point2D) -> u64 {
    let config = Config::new();
    let context = Context::new(&config);
    let solver = Optimize::new(&context);

    let one = Int::from_i64(&context, 1);
    let three = Int::from_i64(&context, 3);

    let d_x = Int::from_i64(&context, destination.x as i64);
    let d_y = Int::from_i64(&context, destination.y as i64);

    let a_x = Int::from_i64(&context, button_a.x as i64);
    let a_y = Int::from_i64(&context, button_a.y as i64);
    let a_count = Int::new_const(&context, "a_count");

    let b_x = Int::from_i64(&context, button_b.x as i64);
    let b_y = Int::from_i64(&context, button_b.y as i64);
    let b_count = Int::new_const(&context, "b_count");

    solver.assert(&((&a_x * &a_count) + (&b_x * &b_count))._eq(&d_x));
    solver.assert(&((&a_y * &a_count) + (&b_y * &b_count))._eq(&d_y));

    let cost = Int::new_const(&context, "cost");
    solver.assert(&((&a_count * &three) + (&b_count * &one))._eq(&cost));
    solver.minimize(&cost);

    if SatResult::Sat == solver.check(&[]) {
        let model = solver.get_model().unwrap();
        model.eval(&cost, true).unwrap().as_i64().unwrap() as u64

    } else {
        0
    }
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut cost = 0;
    loop {
        let Some(line_1) = lines.next() else { break; };
        let line_2 = lines.next().unwrap();
        let line_3 = lines.next().unwrap();

        let button_a = {
            let mut chars = line_1.chars().skip(12);
            let x_mov = parse_int(&mut chars);
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_mov = parse_int(&mut chars);
            Point2D::new(x_mov, y_mov)
        };

        let button_b = {
            let mut chars = line_2.chars().skip(12);
            let x_mov = parse_int(&mut chars);
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_mov = parse_int(&mut chars);
            Point2D::new(x_mov, y_mov)
        };

        let prize = {
            let mut chars = line_3.chars().skip(9);
            let x_pos = parse_int(&mut chars);
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_pos = parse_int(&mut chars);
            Point2D::new(x_pos, y_pos)
        };

        cost += find_min_cost(prize, button_a, button_b);
        lines.next();
    }
    
    cost
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut cost = 0;
    loop {
        let Some(line_1) = lines.next() else { break; };
        let line_2 = lines.next().unwrap();
        let line_3 = lines.next().unwrap();

        let button_a = {
            let mut chars = line_1.chars().skip(12);
            let x_mov = parse_int(&mut chars);
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_mov = parse_int(&mut chars);
            Point2D::new(x_mov, y_mov)
        };

        let button_b = {
            let mut chars = line_2.chars().skip(12);
            let x_mov = parse_int(&mut chars);
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_mov = parse_int(&mut chars);
            Point2D::new(x_mov, y_mov)
        };

        let prize = {
            let mut chars = line_3.chars().skip(9);
            let x_pos = parse_int(&mut chars) + 10000000000000;
            chars.next().unwrap();
            chars.next().unwrap();
            chars.next().unwrap();
            let y_pos = parse_int(&mut chars) + 10000000000000;
            Point2D::new(x_pos, y_pos)
        };

        cost += find_min_cost_z3(prize, button_a, button_b);
        lines.next();
    }
    
    cost
}
