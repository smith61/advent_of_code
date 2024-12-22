use fxhash::{FxHashMap, FxHashSet};

use crate::scaffold::InputParser;


fn generate_next_number(current_number: u64) -> u64 {
    let mut number = current_number;

    number = ((number * 64) ^ number) % 16777216;
    number = ((number / 32) ^ number) % 16777216;
    number = ((number * 2048) ^ number) % 16777216;
    number
}

pub fn part1(mut input: InputParser) -> u64 {
    let mut result = 0;
    while let Some(mut number) = input.next_uint() {
        
        for _ in 0..2000 {
            number = generate_next_number(number);
        }

        result += number;
    }

    result
}

pub fn part2(mut input: InputParser) -> u64 {
    let mut result = 0;
    let mut prices_changes_all = Vec::new();
    let mut sequences = FxHashSet::default();
    while let Some(orig_number) = input.next_uint() {
        let mut number = orig_number;
        let mut previous_price = number % 10;
        
        let mut price_changes = Vec::new();

        let mut prices = Vec::new();
        for _ in 0..2000 {
            number = generate_next_number(number);
            let next_price = number % 10;
            let diff = next_price as i64 - previous_price as i64;
            price_changes.push(diff);
            prices.push(next_price);
            previous_price = next_price;
        }

        let mut change_set = FxHashMap::<Vec<i64>, u64>::default();
        for i in 4..price_changes.len() {
            let changes = &price_changes[(i - 4)..i];
            if changes[0] == -2 && changes[1] == 1 && changes[2] == -1 && changes[3] == 3 {
                //println!("[{}] {:?}: {}", orig_number, changes, prices[i - 1]);
            }

            sequences.insert(changes.to_vec());
            change_set.entry(changes.to_owned())
                      .or_insert(prices[i - 1]);
        }

        prices_changes_all.push(change_set);
    }

    for key in sequences {
        let mut current_result = 0;
        for other in &prices_changes_all {

            if other.contains_key(&key) {
                current_result += other.get(&key).unwrap();
            }
        }
        

        if current_result > result {
            //println!("{:?} = {}", key, current_result);
            result = current_result;
        }
    }

    result
}
