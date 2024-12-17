use itertools::Itertools;
use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day17;

/* 
Register A: 46337277
Register B: 0
Register C: 0

Program: 
2,4, // B = A % 8
1,1, // B = B XOR 000001
7,5, // C = A // 2**B
4,4, // B = B XOR C
1,4, // B = B XOR 00100
0,3, // A = A // 8
5,5, // OUTPUT B % 8
3,0  // JUMP TO START

7,4,2,0,5,0,5,3,7
*/

impl Solution for Day17 {
    fn part_1() -> String {
        let data = load_input(17);
        let mut program = Program::from_data(data);
        let mut output: Vec<u64> = Vec::new();

        while program.instruction_pointer < program.program.len() {
            let opcode = program.program[program.instruction_pointer];
            let operand = program.program[program.instruction_pointer + 1];

            let value = program.get_operand_value(opcode, operand);

            match opcode {
                0 => program.reg_a = program.reg_a / 2_u64.pow(value as u32),
                1 => program.reg_b = program.reg_b ^ value,
                2 => program.reg_b = value % 8,
                3 => {
                    if program.reg_a != 0 {
                        program.instruction_pointer = value as usize
                    }
                }
                4 => program.reg_b = program.reg_b ^ program.reg_c,
                5 => output.push(value % 8),
                6 => program.reg_b = program.reg_a / 2_u64.pow(value as u32),
                7 => program.reg_c = program.reg_a / 2_u64.pow(value as u32),
                _ => panic!("Unexpected opcode '{}'", opcode),
            }
            if opcode != 3 || program.reg_a == 0 {
                program.instruction_pointer += 2
            }
        }

        output.iter().join(",").to_string()
    }

    fn part_2() -> String {
        let data = load_input(17);
        let program = Program::from_data(data);
        let mut targets: Vec<u64> = program.program.iter().map(|n| *n as u64).collect();

        let res = calc_sol(&mut targets, 0);

        res.unwrap().to_string()
    }
}

fn calc_sol(targets: &mut Vec<u64>, global_a: u64) -> Option<u64> {
    let target = match targets.pop() {
        Some(a) => a,
        None => return Some(global_a),
    };

    for i in 0..8 {
        let a = (global_a << 3) + i;
        let mut b = a % 8;
        b = b ^ 1;
        let c = a / 2_u64.pow(b as u32);
        b = b ^ c;
        b = b ^ 4;

        if b % 8 == target {
            match calc_sol(targets, a) {
                Some(a) => return Some(a),
                None => {}
            };
        }
    }

    targets.push(target);
    None
}

#[derive(Debug)]
struct Program {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
    instruction_pointer: usize,
}

impl Program {
    fn get_operand_value(&self, opcode: u8, operand: u8) -> u64 {
        match opcode {
            0 | 2 | 5 | 6 | 7 => self.get_combo_value(operand),
            1 | 3 | 4 => operand as u64,
            _ => panic!("Unknown opcode '{}'", opcode),
        }
    }

    fn get_combo_value(&self, operand: u8) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Found combo operand 7"),
            _ => panic!("Unexpected operand '{}'", operand),
        }
    }

    fn from_data(data: String) -> Self {
        let a_re = Regex::new(r"Register A: ([0-9]+)").unwrap();
        let b_re = Regex::new(r"Register B: ([0-9]+)").unwrap();
        let c_re = Regex::new(r"Register C: ([0-9]+)").unwrap();
        let prog_re = Regex::new(r"Program: ((?:[0-9],)+[0-9])").unwrap();

        let (_, [reg_a]) = a_re.captures_iter(&data).exactly_one().unwrap().extract();
        let (_, [reg_b]) = b_re.captures_iter(&data).exactly_one().unwrap().extract();
        let (_, [reg_c]) = c_re.captures_iter(&data).exactly_one().unwrap().extract();

        let (_, [prog]) = prog_re
            .captures_iter(&data)
            .exactly_one()
            .unwrap()
            .extract();

        Self {
            reg_a: reg_a.parse().expect("Could not parse reg a"),
            reg_b: reg_b.parse().expect("Could not parse reg b"),
            reg_c: reg_c.parse().expect("Could not parse reg c"),
            program: prog
                .split(",")
                .map(|c| c.parse().expect("could not parse number"))
                .collect(),
            instruction_pointer: 0,
        }
    }
}
