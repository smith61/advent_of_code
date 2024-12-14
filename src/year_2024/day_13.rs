
use crate::{scaffold::InputParser, utils::Point2D};

fn parse_machine_description<'a, const OFFSET: isize>(input: &mut InputParser) -> Option<(Point2D, Point2D, Point2D)> {
    let machine_descs = input.next_ints::<6>()?;

    let point_a = Point2D::new(machine_descs[0], machine_descs[1]);
    let point_b = Point2D::new(machine_descs[2], machine_descs[3]);
    let destination = Point2D::new(machine_descs[4] + OFFSET, machine_descs[5] + OFFSET);

    Some((point_a, point_b, destination))
}

fn find_min_cost<const OFFSET: isize>(mut input: InputParser) -> u64 {
    let mut total_cost = 0;
    while let Some((button_a, button_b, destination)) = parse_machine_description::<OFFSET>(&mut input) {
        let b = (destination.y * button_a.x - destination.x * button_a.y) / (button_b.y * button_a.x - button_b.x * button_a.y);
        let a = (destination.x - b * button_b.x) / button_a.x;
        if (button_a.x * a + button_b.x * b) == destination.x &&
           (button_a.y * a + button_b.y * b) == destination.y {

            total_cost += a * 3 + b;
        }
    }

    total_cost as u64
}

pub fn part1(input: InputParser) -> u64 {
    find_min_cost::<0>(input)
}

pub fn part2(input: InputParser) -> u64 {
    find_min_cost::<10000000000000>(input)
}
