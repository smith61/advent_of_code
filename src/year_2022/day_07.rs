
use std::iter::Peekable;

fn walk_directory<'a>(input: &mut Peekable<impl Iterator<Item = &'a str>>, callback: &mut impl FnMut(u64)) -> u64 {
    let mut dir_size = 0;
    loop {
        let Some(command) = input.next() else { break };
        let as_bytes = command.as_bytes();

        assert_eq!(as_bytes[0], b'$');

        if as_bytes[2] == b'c' {
            if as_bytes[5] == b'.' {
                break;
            }

            dir_size += walk_directory(input, callback);

        } else {
            assert_eq!(as_bytes[2], b'l');

            loop {
                let Some(line) = input.peek() else { break; };
                let as_bytes = line.as_bytes();
                if as_bytes[0] == b'$' {
                    break;

                } else if as_bytes[0] != b'd' {
                    let mut size = 0;
                    let mut index = 0;
                    while as_bytes[index] != b' ' {
                        size = (size * 10) + ((as_bytes[index] - b'0') as u64);
                        index += 1;
                    }

                    dir_size += size;
                }

                input.next();
            }
        }
    }

    callback(dir_size);
    dir_size
}

fn walk_directories(input: &str, mut callback: impl FnMut(u64)) -> u64 {
    walk_directory(&mut input.lines().peekable(), &mut callback)
}

pub fn part1(input: &str) -> u64 {
    let mut score = 0;
    walk_directories(
        input,
        |dir_size|  {
            if dir_size <= 100000 {
                score += dir_size;
            }
        });

    score
}

pub fn part2(input: &str) -> u64 {
    const TOTAL_SPACE: u64 = 70000000;
    const NEEDED_SPACE: u64 = 30000000;
    let used_space = walk_directories(input, |_| {});
    let space_to_free = NEEDED_SPACE - (TOTAL_SPACE - used_space);
    
    let mut freed_directory_size = u64::MAX;
    walk_directories(
        input,
        |dir_size| {
            if dir_size >= space_to_free {
                freed_directory_size = std::cmp::min(freed_directory_size, dir_size);
            }
        });

    freed_directory_size
}