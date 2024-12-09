use std::{cmp::Reverse, collections::BinaryHeap};


pub fn part1(input: &str) -> u64 {
    let input = input.trim().as_bytes();
    let right = (input.len() - 1) & !1;

    let mut trail_block = (right / 2, input[right] - b'0');
    let mut free_block = (0, input[1] - b'0');

    let mut disk_offset = 0;
    let mut value = 0;
    let mut checksum = |id: usize, mut length: u8| {
        while length > 0 {
            value += (id as u64) * disk_offset;
            length -= 1;
            disk_offset += 1;
        }
    };

    checksum(0, input[0] - b'0');
    while free_block.0 < trail_block.0 {
        if trail_block.1 == 0 {
            let new_id = trail_block.0 - 1;
            if new_id <= free_block.0 {
                break;
            }

            trail_block = (new_id, input[new_id * 2 + 0] - b'0');
            continue;
        }

        if free_block.1 == 0 {
            let new_id = free_block.0 + 1;
            if new_id >= trail_block.0 {
                break;
            }

            checksum(new_id, input[new_id * 2 + 0] - b'0');
            free_block = (new_id, input[new_id * 2 + 1] - b'0');
            continue;
        }

        let move_size = free_block.1.min(trail_block.1);
        checksum(trail_block.0, move_size);
        free_block.1 -= move_size;
        trail_block.1 -= move_size;
    }

    checksum(trail_block.0, trail_block.1);

    value
}

pub fn part2(input: &str) -> u64 {
    struct Block {
        disk_offset: usize,
        block_size: usize,
        block_id: u64
    }

    let mut file_blocks = Vec::new();
    let mut free_blocks = [0; 10].map(|_| BinaryHeap::default());
    let mut is_free_block = false;
    let mut block_id = 0;
    let mut disk_offset = 0;
    for c in input.bytes() {
        let size = (c - b'0') as usize;
        if is_free_block {
            free_blocks[size].push(Reverse(disk_offset));

        } else {
            file_blocks.push(Block {
                disk_offset,
                block_size: size,
                block_id: block_id
            });

            block_id += 1;
        }

        is_free_block = !is_free_block;
        disk_offset += size;
    }

    for file_index in (0..file_blocks.len()).rev() {
        let file_block = &mut file_blocks[file_index];
        
        let free_block =
            (file_block.block_size..10)
                .flat_map(|size| free_blocks[size].peek().map(|&Reverse(v)| (v, size)))
                .min();

        if let Some((disk_offset, free_size)) = free_block {
            if disk_offset >= file_block.disk_offset {
                continue;
            }
            
            file_block.disk_offset = disk_offset;
            free_blocks[free_size].pop();
            if free_size > file_block.block_size {
                free_blocks[free_size - file_block.block_size].push(Reverse(disk_offset + file_block.block_size));
            }
        }
    }

    file_blocks.iter()
               .map(|block| {
                    let mut value = 0;
                    for idx in 0..block.block_size {
                        value += (block.disk_offset + idx) as u64 * block.block_id;
                    }

                    value
               })
               .sum()
}
