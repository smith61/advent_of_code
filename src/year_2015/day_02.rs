
fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = (u64, u64, u64)> + 'a {
    input
    .lines()
    .map(|line| {
        let bytes = line.as_bytes();
        let mut index = 0;
        let h = {
            let mut val = 0;
            while bytes[index] != b'x' {
                val = (val * 10) + ((bytes[index] - b'0') as u64);
                index += 1;
            }

            val
        };

        index += 1;
        let w = {
            let mut val = 0;
            while bytes[index] != b'x' {
                val = (val * 10) + ((bytes[index] - b'0') as u64);
                index += 1;
            }

            val
        };

        index += 1;
        let l = {
            let mut val = 0;
            while index < bytes.len() {
                val = (val * 10) + ((bytes[index] - b'0') as u64);
                index += 1;
            }

            val
        };
        
        (h, w, l)
    })
}

pub fn part1(input: &str) -> u64 {
    parse_input(input)
    .map(|(h, w, l)| {
        let d1 = h * w;
        let d2 = w * l;
        let d3 = l * h;

        (d1 * 2) + (d2 * 2) + (d3 * 2) + (d1.min(d2).min(d3))
    })
    .sum()
}

pub fn part2(input: &str) -> u64 {
    parse_input(input)
    .map(|(h, w, l)| {
        let d1 = (h + w) * 2;
        let d2 = (w + l) * 2;
        let d3 = (l + h) * 2;
        let v = h * w * l;

        d1.min(d2).min(d3) + v
    })
    .sum()
}