
pub fn part1(input: &str) -> u64 {
    let mut blocks = Vec::new();
    let mut free_block = false;
    let mut block_id = 0;
    for c in input.bytes() {
        let size = c - b'0';
        for _ in 0..size {
            if free_block {
                blocks.push(u64::MAX);

            } else {
                blocks.push(block_id);
            }
        }

        if !free_block {
            block_id += 1;
        }

        free_block = !free_block
    }


    let mut left = 0;
    let mut right = blocks.len() - 1;
    while left < right {
        if blocks[left] != u64::MAX {
            left += 1;
            continue;
        }

        if blocks[right] == u64::MAX {
            right -= 1;
            continue;
        }

        blocks[left] = blocks[right];
        blocks[right] = u64::MAX;
    }

    blocks.iter()
          .enumerate()
          .map(|(idx, v)| {
            if *v != u64::MAX {
                (idx as u64) * *v
            } else {
                0
            }
          })
          .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut free_block = true;
    let mut blocks =
        input.bytes()
             .enumerate()
             .map(|(idx, c)| {
                free_block = !free_block;
                ((idx / 2) as u64, (c - b'0') as u64, free_block)
             })
             .collect::<Vec<_>>();

    for offset in 0..blocks.len() {
        let right = blocks.len() - offset - 1;
        let block = blocks[right];
        if block.2 {
            continue;
        }

        for left in 0..right {
            if !blocks[left].2 ||
                blocks[left].1 < block.1 {

                continue;
            }

            blocks[left].1 -= block.1;
            blocks[right].2 = true;
            blocks.insert(left, block);
            break;
        }
    }

    let mut offset = 0;
    blocks.iter()
            .map(|(idx, size, free_block)| {
                let val = if *free_block {
                    0

                } else {
                    let mut val = 0;
                    for i in 0..*size {
                        val += (offset + i) * *idx;
                    }

                    val
                };

                offset += *size;
                val
            })
            .sum()
}
