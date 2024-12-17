
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

fn calculate_reg_a(current_reg_a: u64, output_index: usize, output: &[u64]) -> Option<u64> {
    for bit in 0..8 {
        let reg_a = (current_reg_a << 3) | bit;
        
        let mut reg_b = reg_a % 8;
        reg_b ^= 1;
        let reg_c = reg_a >> reg_b;
        reg_b ^= 4;
        reg_b ^= reg_c;
        if (reg_b % 8) == output[output_index] {
            if output_index == 0 {
                return Some(reg_a);

            } else if let Some(reg_a) = calculate_reg_a(reg_a, output_index - 1, output) {
                return Some(reg_a);
            }
        }
    }

    None
}

pub fn part2(mut input: InputParser) -> u64 {
    input.next_uints::<3>().unwrap();

    let mut program = Vec::new();
    while let Some(val) = input.next_uint() {
        program.push(val);
    }

    calculate_reg_a(0, program.len() - 1, &program).unwrap()
}
