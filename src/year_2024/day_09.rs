
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

    impl Block {

        fn checksum(&self, size: usize) -> u64 {
            let mut value = 0;
            for idx in 0..size {
                value += (self.disk_offset + idx) as u64 * self.block_id;
            }
    
            value
        }

    }

    let input = input.trim().as_bytes();

    let mut file_blocks = [0; 10].map(|_| Vec::with_capacity(input.len() / 2));

    let mut disk_offset = 0;
    for (block_id, chunk) in input.chunks(2).enumerate() {
        let block_size = (chunk[0] - b'0') as usize;

        file_blocks[block_size].push(Block {
            disk_offset,
            block_size,
            block_id: block_id as u64
        });

        disk_offset += block_size;
        if chunk.len() == 2 {
            disk_offset += (chunk[1] - b'0') as usize;
        }
    }

    let mut value = 0;
    disk_offset = 0;
    for chunk in input.chunks_exact(2) {
        disk_offset += (chunk[0] - b'0') as usize;
        
        let mut free_block_size = (chunk[1] - b'0') as usize;
        loop {
            let mut block_size = 0;
            let mut block_disk_offset = 0;
            for size in 1..=free_block_size {
                if let Some(block) = file_blocks[size].last() {
                    if block.disk_offset <= disk_offset {
                        continue;
                    }

                    if block.disk_offset >= block_disk_offset {
                        block_disk_offset = block.disk_offset;
                        block_size = size;
                    }
                }
            }

            if block_size == 0 {
                break;
            }

            let mut file_block = file_blocks[block_size].pop().unwrap();
            file_block.disk_offset = disk_offset;
            disk_offset += file_block.block_size;
            free_block_size -= file_block.block_size;
            value += file_block.checksum(block_size);
        }

        disk_offset += free_block_size;
    }

    value +=
        file_blocks
            .iter()
            .enumerate()
            .map(|(block_size, fbs)| {
                fbs.iter().map(|block| block.checksum(block_size)).sum::<u64>()
            })
            .sum::<u64>();

    value
}
