use std::{time::Instant, u64};


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
    struct Block {
        disk_offset: u64,
        block_size: u64,
        block_id: u64
    }

    let mut file_blocks = Vec::new();
    let mut free_blocks = Vec::new();
    let mut is_free_block = false;
    let mut block_id = 0;
    let mut disk_offset = 0;
    for c in input.bytes() {
        let size = (c - b'0') as u64;
        if is_free_block {
            free_blocks.push(Block {
                disk_offset,
                block_size: size,
                block_id: u64::MAX
            });

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

        let mut free_index = 0;
        while free_index < free_blocks.len() {
            if free_blocks[free_index].block_size >= file_block.block_size {
                break;
            }

            free_index += 1;
        }

        if (free_index >= free_blocks.len()) ||
           (free_blocks[free_index].disk_offset >= file_block.disk_offset) {

            continue;
        }

        let free_block = &mut free_blocks[free_index];
        file_block.disk_offset = free_block.disk_offset;
        free_block.disk_offset += file_block.block_size;
        free_block.block_size -= file_block.block_size;
    }

    file_blocks.iter()
               .map(|block| {
                    let mut value = 0;
                    for idx in 0..block.block_size {
                        value += (block.disk_offset + idx) * block.block_id;
                    }

                    value
               })
               .sum()
}
