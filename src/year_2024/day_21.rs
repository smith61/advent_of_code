use std::time::Instant;

use fxhash::{FxHashMap, FxHashSet};

use crate::{main, scaffold::InputParser, utils::Vector2};

/*

--+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
    */

fn number_position(num: u8) -> Vector2 {
    match num {
        b'0' => Vector2::new(1, 3),
        b'1' => Vector2::new(0, 2),
        b'2' => Vector2::new(1, 2),
        b'3' => Vector2::new(2, 2),
        b'4' => Vector2::new(0, 1),
        b'5' => Vector2::new(1, 1),
        b'6' => Vector2::new(2, 1),
        b'7' => Vector2::new(0, 0),
        b'8' => Vector2::new(1, 0),
        b'9' => Vector2::new(2, 0),
        b'A' => Vector2::new(2, 3),
        num => panic!("Invalid num: {}", num as char)
    }
}

/*

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

fn arrow_position(arrow: u8) -> Vector2 {
    match arrow {
        b'<' => Vector2::new(0, 1),
        b'v' => Vector2::new(1, 1),
        b'>' => Vector2::new(2, 1),
        b'^' => Vector2::new(1, 0),
        b'A' => Vector2::new(2, 0),
        c => panic!("Invalid character {}", c as char)
    }
}

fn try_horizonal_sequence_num_pad(current_position: Vector2, next_position: Vector2, num_robots: u64, memo: &mut FxHashMap<(u64, Vec<u8>), u64>) -> Option<u64> {
    if current_position.y() == 3 && next_position.x() == 0 {
        return None;
    }

    let diff = next_position - current_position;

    let mut new_sequence = Vec::new();
    if diff.x() < 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'<'));
     
    } else if diff.x() > 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'>'));
    }

    if diff.y() < 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'^'));

    } else if diff.y() > 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'v'));
    }

    new_sequence.push(b'A');
    Some(get_sequence_button_count(new_sequence, num_robots, memo))
}

fn try_vertical_sequence_num_pad(current_position: Vector2, next_position: Vector2, num_robots: u64, memo: &mut FxHashMap<(u64, Vec<u8>), u64>) -> Option<u64> {
    if current_position.x() == 0 && next_position.y() == 3 {
        return None;
    }

    let diff = next_position - current_position;

    let mut new_sequence = Vec::new();
    if diff.y() < 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'^'));

    } else if diff.y() > 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'v'));
    }

    if diff.x() < 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'<'));
     
    } else if diff.x() > 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'>'));
    }

    new_sequence.push(b'A');
    Some(get_sequence_button_count(new_sequence, num_robots, memo))


}

fn get_minimum_button_presses_num_pad(from_value: u8, to_value: u8, num_robots: u64) -> u64 {
    let mut memo = FxHashMap::default();


    let current_position = number_position(from_value);
    let next_position = number_position(to_value);

    let horizonal = try_horizonal_sequence_num_pad(current_position, next_position, num_robots, &mut memo);
    let vertical = try_vertical_sequence_num_pad(current_position, next_position, num_robots, &mut memo);

    match (horizonal, vertical) {
        (Some(l), Some(r)) => l.min(r),
        (Some(l), None) => l,
        (None, Some(r)) => r,
        (None, None) => unimplemented!()
    }
}

fn generate_main_robot_string(current_position: Vector2, next_position: Vector2) -> Vec<u8> {
    let mut result = Vec::new();

    let diff = next_position - current_position;
    if current_position.y() == 3 && next_position.x() == 0 {
        result.extend((0..diff.y().abs()).map(|_| b'^'));
        result.extend((0..diff.x().abs()).map(|_| b'<'));

    } else if current_position.x() == 0 && next_position.y() == 3 {
        result.extend((0..diff.x().abs()).map(|_| b'>'));
        result.extend((0..diff.y().abs()).map(|_| b'v'));

    } else {
        if diff.x() < 0 {
            if diff.y() < 0 {
                result.extend((0..diff.x().abs()).map(|_| b'<'));
                result.extend((0..diff.y().abs()).map(|_| b'^'));

            } else if diff.y() > 0 {
                result.extend((0..diff.x().abs()).map(|_| b'<'));
                result.extend((0..diff.y().abs()).map(|_| b'v'));

            } else {
                result.extend((0..diff.x().abs()).map(|_| b'<'));
            }

        } else if diff.x() > 0 {
            if diff.y() < 0 {
                result.extend((0..diff.x().abs()).map(|_| b'>'));
                result.extend((0..diff.y().abs()).map(|_| b'^'));

            } else if diff.y() > 0 {
                result.extend((0..diff.y().abs()).map(|_| b'v'));
                result.extend((0..diff.x().abs()).map(|_| b'>'));

            } else {
                result.extend((0..diff.x().abs()).map(|_| b'>'));
            }

        } else {
            if diff.y() < 0 {
                result.extend((0..diff.y().abs()).map(|_| b'^'));
        
            } else if diff.y() > 0 {
                result.extend((0..diff.y().abs()).map(|_| b'v'));
            }
        }
    }

    result.push(b'A');
    result
}

fn try_horizonal_sequence(current_position: Vector2, next_position: Vector2, remaining_robot_count: u64, memo: &mut FxHashMap<(u64, Vec<u8>), u64>) -> Option<u64> {
    if current_position.y() == 0 && next_position.x() == 0 {
        return None;
    }

    let diff = next_position - current_position;

    let mut new_sequence = Vec::new();
    if diff.x() < 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'<'));
     
    } else if diff.x() > 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'>'));
    }

    if diff.y() < 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'^'));

    } else if diff.y() > 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'v'));
    }

    new_sequence.push(b'A');
    Some(get_sequence_button_count(new_sequence, remaining_robot_count - 1, memo))
}

fn try_vertical_sequence(current_position: Vector2, next_position: Vector2, remaining_robot_count: u64, memo: &mut FxHashMap<(u64, Vec<u8>), u64>) -> Option<u64> {
    if current_position.x() == 0 && next_position.y() == 0 {
        return None;
    }

    let diff = next_position - current_position;

    let mut new_sequence = Vec::new();
    if diff.y() < 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'^'));

    } else if diff.y() > 0 {
        new_sequence.extend((0..diff.y().abs()).map(|_| b'v'));
    }

    if diff.x() < 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'<'));
     
    } else if diff.x() > 0 {
        new_sequence.extend((0..diff.x().abs()).map(|_| b'>'));
    }

    new_sequence.push(b'A');
    Some(get_sequence_button_count(new_sequence, remaining_robot_count - 1, memo))
}

fn get_sequence_button_count(sequence: Vec<u8>, remaining_robot_count: u64, memo: &mut FxHashMap<(u64, Vec<u8>), u64>) -> u64 {
    if remaining_robot_count == 0 {
        return sequence.len() as u64;
    }

    if memo.contains_key(&(remaining_robot_count, sequence.clone())) {
        return *memo.get(&(remaining_robot_count, sequence.clone())).unwrap();
    }

    let mut current_position = arrow_position(b'A');
    let mut button_count = 0;
    for &c in &sequence {
        let next_position = arrow_position(c);
        let horizonal = try_horizonal_sequence(current_position, next_position, remaining_robot_count, memo);
        let vertical = try_vertical_sequence(current_position, next_position, remaining_robot_count, memo);

        button_count += match (horizonal, vertical) {
            (Some(l), Some(r)) => l.min(r),
            (Some(l), None) => l,
            (None, Some(r)) => r,
            (None, None) => unimplemented!()
        };

        current_position = next_position;
    }

    assert_eq!(current_position, arrow_position(b'A'));

    memo.insert((remaining_robot_count, sequence), button_count);
    button_count
}

fn generate_secondary_robot_string(prev_robot: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current_position = arrow_position(b'A');
    for &c in prev_robot {
        let next_position = arrow_position(c);
        let diff = next_position - current_position;
        
        if diff.x() < 0 {
            result.extend((0..diff.x().abs()).map(|_| b'<'));
         
        } else if diff.x() > 0 {
            result.extend((0..diff.x().abs()).map(|_| b'>'));
        }

        if diff.y() < 0 {
            result.extend((0..diff.y().abs()).map(|_| b'^'));
    
        } else if diff.y() > 0 {
            result.extend((0..diff.y().abs()).map(|_| b'v'));
        }

        result.push(b'A');
        current_position = next_position;
    }

    result
}

pub fn part1(input: &str) -> u64 {
    let input = input.trim();

    let mut result = 0;
    for line in input.lines() {
        let chars = line.as_bytes();
        let mut num_value = 0;
        let mut button_count = 0;

        let mut current_position = b'A';
        for &c in chars {
            if c != b'A' {
                num_value = (num_value * 10) + ((c - b'0') as u64);
            }

            button_count += get_minimum_button_presses_num_pad(current_position, c, 2);
            
            current_position = c;
        }

        println!("{} {}", num_value, button_count);
        result += num_value * button_count;
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let input = input.trim();

    let mut result = 0;
    for line in input.lines() {
        let chars = line.as_bytes();
        let mut num_value = 0;
        let mut button_count = 0;

        let mut current_position = b'A';
        for &c in chars {
            if c != b'A' {
                num_value = (num_value * 10) + ((c - b'0') as u64);
            }

            button_count += get_minimum_button_presses_num_pad(current_position, c, 25);
            
            current_position = c;
        }

        println!("{} {}", num_value, button_count);
        result += num_value * button_count;
    }

    result
}
