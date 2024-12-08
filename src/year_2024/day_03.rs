fn matches(chars: impl Iterator<Item = char>, val: &str) -> bool {
    chars.take(val.len()).eq(val.chars())
}

fn skip(chars: &mut impl Iterator<Item = char>, mut count: usize) {
    while count != 0 {
        chars.next();
        count -= 1;
    }
}

fn try_parse_mul(mut chars: impl Iterator<Item = char>) -> Option<u64> {
    let mut has_value = false;
    let mut first_value = 0;
    loop {
        if let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                has_value = true;
                first_value = first_value * 10 + (c as u64 - '0' as u64);

            } else if c == ',' {
                break;

            } else {
                return None;
            }
    
        } else {
            return None;
        }
    }

    if !has_value {
        return None;
    }

    has_value = false;
    let mut second_value = 0;
    loop {
        if let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                has_value = true;
                second_value = second_value * 10 + (c as u64 - '0' as u64);

            } else if c == ')' {
                break;

            } else {
                return None;
            }
    
        } else {
            return None;
        }
    }

    if !has_value {
        return None;
    }

    return Some(first_value * second_value);
}

pub fn part1(input: &str) -> u64 {
    let mut result = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == 'm' {
            if matches(chars.clone(), "ul(") {
                skip(&mut chars, "ul(".len());
                if let Some(val) = try_parse_mul(chars.clone()) {
                    result += val;
                }
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let mut result = 0;
    let mut chars = input.chars();

    let mut discard_value = false;
    while let Some(c) = chars.next() {
        if c == 'm' {
            if matches(chars.clone(), "ul(") {
                skip(&mut chars, "ul(".len());
                if let Some(val) = try_parse_mul(chars.clone()) {
                    if !discard_value {
                        result += val;
                    }
                }
            }

        } else if c == 'd' {
            if matches(chars.clone(), "o()") {
                discard_value = false;
                skip(&mut chars, "o()".len());

            } else if matches(chars.clone(), "on't()") {
                discard_value = true;
                skip(&mut chars, "on't()".len());
            }
        }
    }

    result
}