
pub fn part1(input: &str) -> u64 {
    let mut total_chars = 0;
    let mut parsed_chars = 0;
    for line in input.trim().lines() {
        let line = line.as_bytes();
        total_chars += line.len();
        
        let mut index = 0;
        while index < line.len() {
            parsed_chars += 1;
            if line[index] == b'\\' {
                if line[index + 1] == b'x' {
                    index += 4;

                } else {
                    index += 2;
                }

            } else {
                index += 1;
            }
        }

        parsed_chars -= 2;
    }

    (total_chars - parsed_chars) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut total_chars = 0;
    let mut parsed_chars = 0;
    for line in input.trim().lines() {
        let line = line.as_bytes();
        parsed_chars += line.len();
        
        let mut index = 0;
        while index < line.len() {
            if line[index] == b'"' ||
               line[index] == b'\\' {

                total_chars += 2;

            } else {
                total_chars += 1;
            }

            index += 1;
        }

        total_chars += 2;
    }

    (total_chars - parsed_chars) as u64
}