
use std::collections::VecDeque;

fn run_simulation<const DECRYPTION_KEY: i64, const SHUFFLE_COUNT: u32>(input: &str) -> i64 {
    let nums =
        input
        .lines()
        .map(|line| i64::from_str_radix(line, 10).unwrap())
        .map(|n| n * DECRYPTION_KEY)
        .collect::<Vec<_>>();
    
    let mut idxs = (0..nums.len()).collect::<VecDeque<_>>();
    for _ in 0..SHUFFLE_COUNT {
        for (sort_index, &num) in nums.iter().enumerate() {
            let num_pos = idxs.iter().position(|&idx| idx == sort_index).unwrap();
            idxs.remove(num_pos);
            let new_num_pos = ((num_pos as i64) + num).rem_euclid(idxs.len() as i64) as usize;
            idxs.insert(new_num_pos, sort_index);
        }
    }
    
    let zero_index = idxs.iter().position(|&idx| nums[idx] == 0).unwrap();
    let mut val = 0;
    for index in [1000, 2000, 3000] {
        let r_index = (zero_index + index) % nums.len();
        val += nums[idxs[r_index]];
    }

    val
}

pub fn part1(input: &str) -> i64 {
    run_simulation::<1, 1>(input)
}

pub fn part2(input: &str) -> i64 {
    run_simulation::<811589153, 10>(input)
}