use crate::utils::Point2D;
use crate::utils::z3::*;

use fxhash::FxHashSet;

use z3::{Config, Context, Optimize, SatResult, ast::Int};

fn parse_int(bytes: &[u8], index: &mut usize) -> isize {
    let mut sign = 1;
    if bytes[*index] == b'-' {
        sign = -1;
        *index += 1;
    }

    let mut val = 0;
    while *index < bytes.len() && bytes[*index] >= b'0' && bytes[*index] <= b'9' {
        val = (val * 10) + ((bytes[*index] - b'0') as isize);
        *index += 1;
    }

    val * sign
}

fn input_iterator<'a>(input: &'a str) -> impl Iterator<Item = (Point2D, Point2D)> + 'a {
    input
    .lines()
    .map(|line| {
        let bytes = line.as_bytes();
        let mut index = "Sensor at x=".len();
        let s_x = parse_int(&bytes, &mut index);

        index += ", y=".len();
        let s_y = parse_int(bytes, &mut index);

        index += ": closest beacon is at x=".len();
        let b_x = parse_int(bytes, &mut index);

        index += ", y=".len();
        let b_y = parse_int(bytes, &mut index);

        (Point2D::new(s_x, s_y), Point2D::new(b_x, b_y))
    })
}

fn get_ranges(sensors: &[(Point2D, Point2D)], y_index: isize) -> Vec<(isize, isize)> {
    let mut ranges = vec![];
    for &(s_pos, b_pos) in sensors {
        let max_distance = (b_pos - s_pos).manhattan_distance();
        let rem_distance = max_distance - (y_index - s_pos.y).abs();
        if rem_distance < 0 {
            continue;
        }

        ranges.push((s_pos.x - rem_distance, s_pos.x + rem_distance));
    }

    ranges.sort();
    let mut condensed_ranges = vec![];
    for range in ranges {
        if let Some((_, old_to)) = condensed_ranges.last_mut() {
            let (from, to) = range;
            if from <= *old_to {
                *old_to = std::cmp::max(*old_to, to);
                continue;
            }
        }

        condensed_ranges.push(range);
    }

    condensed_ranges
}

pub fn part1(input: &str) -> u64 {
    let sensors =
        input_iterator(input)
        .collect::<Vec<_>>();

    let mut total_count =
        get_ranges(&sensors, 2000000)
            .into_iter()
            .map(|(from, to)| (to - from) as u64)
            .sum();

    let mut seen_b_pos = FxHashSet::default();
    for (_, b_pos) in sensors {
        if seen_b_pos.contains(&b_pos) {
            continue;
        }

        seen_b_pos.insert(b_pos);
        if b_pos.y == 2000000 {
            total_count -= 1;
        }
    }

    total_count
}

pub fn part2(input: &str) -> i64 {
    let config = Config::new();
    let context = Context::new(&config);
    let solver = Optimize::new(&context);

    let zero = Int::from_i64(&context, 0);
    let s_x = Int::new_const(&context, "s_x");
    let s_y = Int::new_const(&context, "s_y");
    for (s_pos, b_pos) in input_iterator(input) {
        let zs_x = Int::from_i64(&context, s_pos.x as i64);
        let zs_y = Int::from_i64(&context, s_pos.y as i64);

        let d_x = (b_pos.x - s_pos.x).abs() as i64;
        let d_y = (b_pos.y - s_pos.y).abs() as i64;
        let zd = Int::from_i64(&context, d_x + d_y);

        let abs_zsd_x = z3_abs(&context, &s_x - &zs_x);
        let abs_zsd_y = z3_abs(&context, &s_y - &zs_y);
        let abs_zsd = &abs_zsd_x + abs_zsd_y;
        solver.assert(&abs_zsd.gt(&zd));
    }

    let upper_bound = Int::from_i64(&context, 4000000);
    solver.assert(&s_x.ge(&zero));
    solver.assert(&s_x.le(&upper_bound));
    solver.assert(&s_y.ge(&zero));
    solver.assert(&s_y.le(&upper_bound));

    assert_eq!(solver.check(&[]), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let x = model.eval(&s_x, true).unwrap().as_i64().unwrap();
    let y = model.eval(&s_y, true).unwrap().as_i64().unwrap();

    x * 4000000 + y
}
