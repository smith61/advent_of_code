
use crate::utils::Point2D;

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

fn parse_machine_description<'a, const OFFSET: isize>(lines: &mut impl Iterator<Item = &'a str>) -> Option<(Point2D, Point2D, Point2D)> {
    let Some(line_1) = lines.next() else { return None; };
    let Some(line_2) = lines.next() else { return None; };
    let Some(line_3) = lines.next() else { return None; };
    lines.next();

    let point_a = {
        let bytes = line_1.as_bytes();
        let mut index = b"Button A: X+".len();
        let x = parse_int(bytes, &mut index);

        index += b", Y+".len();
        let y = parse_int(bytes, &mut index);
        Point2D::new(x, y)
    };

    let point_b = {
        let bytes = line_2.as_bytes();
        let mut index = b"Button B: X+".len();
        let x = parse_int(bytes, &mut index);

        index += b", Y+".len();
        let y = parse_int(bytes, &mut index);
        Point2D::new(x, y)
    };

    let destination = {
        let bytes = line_3.as_bytes();
        let mut index = b"Prize: X=".len();
        let x = parse_int(bytes, &mut index) + OFFSET;

        index += b", Y=".len();
        let y = parse_int(bytes, &mut index) + OFFSET;
        Point2D::new(x, y)
    };

    Some((point_a, point_b, destination))
}

fn find_min_cost<const OFFSET: isize>(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut total_cost = 0;
    while let Some((button_a, button_b, destination)) = parse_machine_description::<OFFSET>(&mut lines) {
        let b = (destination.y * button_a.x - destination.x * button_a.y) / (button_b.y * button_a.x - button_b.x * button_a.y);
        let a = (destination.x - b * button_b.x) / button_a.x;
        if (button_a.x * a + button_b.x * b) == destination.x &&
           (button_a.y * a + button_b.y * b) == destination.y {

            total_cost += a * 3 + b;
        }
    }

    total_cost as u64
}

pub fn part1(input: &str) -> u64 {
    find_min_cost::<0>(input)
}

pub fn part2(input: &str) -> u64 {
    find_min_cost::<10000000000000>(input)
}
