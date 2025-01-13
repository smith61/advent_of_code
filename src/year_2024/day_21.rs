
/*
mod generator {

    use crate::utils::{Matrix3DOwned, Vector2};

    /*
    --+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
    |   |0/^| A |
    +---+---+---+
    | < | v | > |
    +---+---+---+
    */

    #[derive(Debug, Clone, Copy)]
    enum KeypadButton {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Up,
        Left,
        Down,
        Right,
        A,
        Invalid
    }

    impl KeypadButton {

        pub fn from_char(c: u8) -> Self {
            use KeypadButton::*;
            match c {
                b'0' => Zero,
                b'1' => One,
                b'2' => Two,
                b'3' => Three,
                b'4' => Four,
                b'5' => Five,
                b'6' => Six,
                b'7' => Seven,
                b'8' => Eight,
                b'9' => Nine,
                b'A' => A,
                b'<' => Left,
                b'^' => Up,
                b'>' => Right,
                b'v' => Down,
                _ => panic!("Invalid character: {}", c)
            }
        }

        pub fn get_button_id(self) -> usize {
            use KeypadButton::*;
            match self {
                Seven => 0,
                Eight => 1,
                Nine => 2,
                Four => 3,
                Five => 4,
                Six => 5,
                One => 6,
                Two => 7,
                Three => 8,
                Invalid => 9,
                Zero | Up => 10,
                A => 11,
                Left => 12,
                Down => 13,
                Right => 14,
            }
        }

        pub fn get_button_location(self) -> Vector2 {
            use KeypadButton::*;
            match self {
                Seven => Vector2::new(0, 0),
                Eight => Vector2::new(1, 0),
                Nine => Vector2::new(2, 0),
                Four => Vector2::new(0, 1),
                Five => Vector2::new(1, 1),
                Six => Vector2::new(2, 1),
                One => Vector2::new(0, 2),
                Two => Vector2::new(1, 2),
                Three => Vector2::new(2, 2),
                Invalid => Vector2::new(0, 3),
                Zero | Up => Vector2::new(1, 3),
                A => Vector2::new(2, 3),
                Left => Vector2::new(0, 4),
                Down => Vector2::new(1, 4),
                Right => Vector2::new(2, 4),
            }
        }

    }

    fn count_navigation_sequence(first_sequence: (KeypadButton, u64), second_sequence: (KeypadButton, u64), remaining_keypad_count: usize, memo: &mut Matrix3DOwned<u64>) -> u64 {
        let (first_end_key, first_button_count) = if first_sequence.1 == 0 {
            (KeypadButton::A, 0)

        } else {
            let first_end_key = first_sequence.0;
            let first_button_count =
                count_button_presses(
                    KeypadButton::A,
                    first_end_key,
                    remaining_keypad_count,
                    memo);

            (first_end_key, first_button_count + first_sequence.1 - 1)
        };

        let (second_end_key, second_button_count) = if second_sequence.1 == 0 {
            (first_end_key, 0)

        } else {
            let second_end_key = second_sequence.0;
            let second_button_count =
                count_button_presses(
                    first_end_key,
                    second_end_key,
                    remaining_keypad_count,
                    memo);

            (second_end_key, second_button_count + second_sequence.1 - 1)
        };

        let reverse_second_button_count =
            count_button_presses(
                second_end_key,
                KeypadButton::A,
                remaining_keypad_count,
                memo);

        first_button_count + second_button_count + reverse_second_button_count
    }

    fn count_button_presses(from: KeypadButton, to: KeypadButton, remaining_keypad_count: usize, memo: &mut Matrix3DOwned<u64>) -> u64 {
        let from_position = from.get_button_location();
        let to_position = to.get_button_location();
        let invalid_position = KeypadButton::Invalid.get_button_location();

        let diff = to_position - from_position;
        if remaining_keypad_count == 0 {
            return (diff.manhattan_distance() + 1) as u64;
        }

        let memo_key = (remaining_keypad_count, from.get_button_id(), to.get_button_id());
        if memo[memo_key] == u64::MAX {
            let horizontal_sequence = if diff.x() < 0 {
                (KeypadButton::Left, -diff.x() as u64)

            } else {
                (KeypadButton::Right, diff.x() as u64)
            };

            let vertical_sequence = if diff.y() < 0 {
                (KeypadButton::Up, -diff.y() as u64)

            } else {
                (KeypadButton::Down, diff.y() as u64)
            };
            
            let horizontal_count = {
                if from_position.y() != invalid_position.y() ||
                to_position.x() != invalid_position.x() {

                    let horizontal_sequence_count =
                        count_navigation_sequence(
                            horizontal_sequence,
                            vertical_sequence,
                            remaining_keypad_count - 1,
                            memo);

                    Some(horizontal_sequence_count)

                } else {
        
                    None
                }
            };
            
            let vertical_count = {
                if from_position.x() != invalid_position.x() ||
                to_position.y() != invalid_position.y() {

                    let vertical_sequence_count =
                        count_navigation_sequence(
                            vertical_sequence,
                            horizontal_sequence,
                            remaining_keypad_count - 1,
                            memo);

                    Some(vertical_sequence_count)

                } else {
        
                    None
                }
            };

            memo[memo_key] = match (horizontal_count, vertical_count) {
                (Some(l), Some(r)) => if l < r { l } else { r },
                (Some(l), None) => l,
                (None, Some(r)) => r,
                (None, None) => panic!("Both paths were invalid")
            };
        }

        memo[memo_key]
    }

    fn build_key_map<const KEYPAD_COUNT: usize>() -> Matrix3DOwned<u64> {
        let mut memo = Matrix3DOwned::new(KEYPAD_COUNT, 15, 15);
        memo.backing_store_mut().fill(u64::MAX);

        let top_keyboard_chars = b"0123456789A";

        let mut s = 0;
        while s < top_keyboard_chars.len() {
            let mut e = 0;
            while e < top_keyboard_chars.len() {
                count_button_presses(
                    KeypadButton::from_char(top_keyboard_chars[s]),
                    KeypadButton::from_char(top_keyboard_chars[e]),
                    KEYPAD_COUNT - 1,
                    &mut memo);

                e += 1;
            }

            s += 1;
        }
        
        memo
    }

    fn get_keypress_count<const KEYPAD_COUNT: usize>(input: &str, key_map: &Matrix3DOwned<u64>) -> u64 {
        let chars = input.as_bytes();
        let mut button_count = 0;
        let mut current_position = KeypadButton::A;
        for &c in chars {
            let next_position = KeypadButton::from_char(c);
            button_count += key_map[(KEYPAD_COUNT - 1, current_position.get_button_id(), next_position.get_button_id())];
            current_position = next_position;
        }

        button_count
    }

    pub fn generate_lookup_table<const KEYPAD_COUNT: usize>() {
        let key_map = build_key_map::<KEYPAD_COUNT>();
        println!("[");
        for number in 0..1000 {
            let number_str = format!("{:03}A", number);
            print!("{}", get_keypress_count::<KEYPAD_COUNT>(&number_str, &key_map) * number);
            if number != 999 {
                println!(",");

            } else {
                println!();
            }
        }

        println!("]");
    }

}
*/

const LUT_PART1: [u64; 1000] = include!("day_21_lut/lut_part1.txt");
const LUT_PART2: [u64; 1000] = include!("day_21_lut/lut_part2.txt");

fn solve(input: &str, lut: &[u64; 1000]) -> u64 {
    let input = input.trim();

    let mut result = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();

        assert!(bytes.len() == 4);
        
        let number =
            (((bytes[0] - b'0') as usize) * 100) +
            (((bytes[1] - b'0') as usize) *  10) +
            (((bytes[2] - b'0') as usize) *   1);

        result += lut[number];
    }

    result
}

pub fn part1(input: &str) -> u64 {
    solve(input, &LUT_PART1)
}

pub fn part2(input: &str) -> u64 {
    solve(input, &LUT_PART2)
}
