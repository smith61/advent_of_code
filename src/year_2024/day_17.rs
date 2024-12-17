
use crate::scaffold::InputParser;

pub fn part1(mut input: InputParser) -> String {
    let mut reg_a = input.next_uint().unwrap();
    let mut reg_b = input.next_uint().unwrap();
    let mut reg_c = input.next_uint().unwrap();
    let mut ip = 0;

    let mut program = Vec::new();
    while let Some(val) = input.next_uint() {
        program.push(val);
    }
    
    let mut output = String::new();
    loop {
        if ip >= (program.len() - 1) {
            break;
        }

        let op = program[ip];
        let combo = program[ip + 1];
        ip += 2;
        let param = if (op == 1) || (op == 3) {
            combo

        } else if combo == 4 {
            reg_a
    
        } else if combo == 5 {
            reg_b
    
        } else if combo == 6 {
            reg_c

        } else {
            combo
        };

        match op {
            0 => reg_a >>= param,
            1 => reg_b ^= param,
            2 => reg_b = param % 8,
            3 => {
                if reg_a != 0 {
                    ip = param as usize;
                }
            },
            4 => reg_b ^= reg_c,
            5 => {
                if !output.is_empty() {
                    output += ",";
                }

                output += &format!("{}", (param % 8) as u8);
            },
            6 => reg_b = reg_a >> param,
            7 => reg_c = reg_a >> param,
            op => panic!("Invalid opcode {}", op)
        }
    }
    
    output
}

fn assert_valid_program(program: &[u8]) {
    if program.len() < 4 ||
       (program.len() % 2) != 0 {

        panic!("Incorrect program length");
    }

    if !program.ends_with(&[5, 5, 3, 0]) {
        panic!("Program does not end with 'out b, jnz 0'");
    }

    let mut reg_a_shifted = false;
    let mut reg_b_initialized = false;
    let mut reg_c_initialized = false;
    for pair in program[..(program.len() - 4)].chunks_exact(2) {
        let op = pair[0];
        let param = if (op == 1) || (op == 3) {
            pair[1]

        } else if pair[1] == 4 {
            0

        } else if pair[1] == 5 {
            if !reg_b_initialized {
                panic!("Attempted to use unitialized register b");
            }

            0

        } else if pair[1] == 6 {
            if !reg_c_initialized {
                panic!("Attempted to use unitialized register c");
            }

            0

        } else {
            pair[1]
        };

        match op {
            0 => {
                if reg_a_shifted {
                    panic!("Attempted to shift register a multiple times.");
                }

                reg_a_shifted = true;
                if param != 3 {
                    panic!("Attempted to shift register a by value other than 3");
                }
            },
            1 => reg_b_initialized = true,
            2 => reg_b_initialized = true,
            3 => {
                panic!("Program contains unexpected 'jnz'")
            },
            4 => reg_b_initialized = true,
            5 => {
                panic!("Program contains unexpected 'out'.")
            },
            6 => reg_b_initialized = true,
            7 => reg_c_initialized = true,
            op => panic!("Invalid opcode {}", op)
        }
    }
}

fn simulate_single_iteration(mut reg_a: u64, program: &[u8]) -> u8 {
    let mut reg_b = 0;
    let mut reg_c = 0;

    for pair in program[..(program.len() - 4)].chunks_exact(2) {
        let op = pair[0];
        let param = if op == 1 {
            pair[1] as u64

        } else if pair[1] == 4 {
            reg_a

        } else if pair[1] == 5 {
            reg_b

        } else if pair[1] == 6 {
            reg_c

        } else {
            pair[1] as u64
        };

        match op {
            0 => reg_a >>= param,
            1 => reg_b ^= param,
            2 => reg_b = param % 8,
            4 => reg_b ^= reg_c,
            6 => reg_b = reg_a >> param,
            7 => reg_c = reg_a >> param,
            op => panic!("Invalid opcode {}", op)
        }
    }

    (reg_b % 8) as u8
}

fn calculate_reg_a(current_reg_a: u64, program: &[u8], output_index: usize) -> Option<u64> {
    for bit in 0..8 {
        let reg_a = (current_reg_a << 3) | bit;
        let result = simulate_single_iteration(reg_a, program);

        if result != program[output_index] {
            continue;
        }

        if output_index == 0 {
            return Some(reg_a);

        } else if let Some(reg_a) = calculate_reg_a(reg_a, program, output_index - 1) {
            return Some(reg_a);
        }
    }

    None
}

pub fn part2(mut input: InputParser) -> u64 {
    input.next_uints::<3>().unwrap();

    let mut program = Vec::new();
    while let Some(val) = input.next_uint() {
        program.push(val as u8);
    }

    assert_valid_program(&program);
    calculate_reg_a(0, &program, program.len() - 1).unwrap()
}
