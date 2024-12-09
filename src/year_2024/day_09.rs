use std::{cmp::Reverse, collections::BinaryHeap};


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
