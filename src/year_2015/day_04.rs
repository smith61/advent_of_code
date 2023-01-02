
pub fn part1(_input: &str) -> u64 {
    /*let (mut buffer, start_index) = {
        let bytes = input.trim().as_bytes();
        let mut buffer = Vec::with_capacity(bytes.len() + 20);
        buffer.extend_from_slice(bytes);
        (buffer, bytes.len())
    };

    let mut index = start_index;
    buffer.push(b'0');
    let mut val = 0;
    loop {
        let hash = md5::compute(&buffer[..]);
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            break;
        }

        for i in (start_index..=index).rev() {
            if buffer[i] == b'9' {
                if i == start_index {
                    buffer[i] = b'1';
                    index += 1;
                    buffer.push(b'0');
                    break;

                } else {
                    buffer[i] = b'0';
                }

            } else {
                buffer[i] += 1;
                break;
            }
        }

        val += 1;
    }

    val*/

    u64::MAX
}

pub fn part2(_input: &str) -> u64 {
    /*let (mut buffer, start_index) = {
        let bytes = input.trim().as_bytes();
        let mut buffer = Vec::with_capacity(bytes.len() + 20);
        buffer.extend_from_slice(bytes);
        (buffer, bytes.len())
    };

    let mut index = start_index;
    buffer.push(b'0');
    let mut val = 0;
    loop {
        let hash = md5::compute(&buffer[..]);
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            break;
        }

        for i in (start_index..=index).rev() {
            if buffer[i] == b'9' {
                if i == start_index {
                    buffer[i] = b'1';
                    index += 1;
                    buffer.push(b'0');
                    break;

                } else {
                    buffer[i] = b'0';
                }

            } else {
                buffer[i] += 1;
                break;
            }
        }

        val += 1;
    }

    val*/

    u64::MAX
}