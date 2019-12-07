const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JUMP_IF_TRUE: u32 = 5;
const JUMP_IF_FALSE: u32 = 6;
const LESS_THAN: u32 = 7;
const EQUALS: u32 = 8;
const HALT: u32 = 99;

const POSITION: u32 = 0;
const IMMEDIATE: u32 = 1;

pub fn run(mut memory: Vec<i32>, mut inputs: Vec<i32>) -> i32 {
    let mut instruction_pointer = 0;
    let mut output = 0;

    'main: loop {
        let instruction: String = format!("0000{}", memory[instruction_pointer].to_string())
            .chars()
            .rev()
            .collect();

        let opcode: u32 = instruction[..2]
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();

        let mut parameters = vec![0; get_parameter_count(opcode)];

        for i in 0..parameters.len() {
            let value = memory[instruction_pointer + i + 1];

            if location_parameter(opcode, i) {
                parameters[i] = value;
                continue;
            }

            let mode: u32 = instruction
                .chars()
                .nth(2 + i)
                .unwrap()
                .to_digit(10)
                .unwrap();

            parameters[i] = match mode {
                POSITION => memory[value as usize],
                IMMEDIATE => value,
                _ => panic!("An unknown mode was encountered"),
            };
        }

        match opcode {
            ADD => {
                memory[parameters[2] as usize] = parameters[0] + parameters[1];
            }
            MULTIPLY => {
                memory[parameters[2] as usize] = parameters[0] * parameters[1];
            }
            INPUT => {
                memory[parameters[0] as usize] = inputs.pop().unwrap();
            }
            OUTPUT => {
                output = parameters[0];
            }
            JUMP_IF_TRUE => {
                if parameters[0] != 0 {
                    instruction_pointer = parameters[1] as usize;
                    continue 'main;
                }
            }
            JUMP_IF_FALSE => {
                if parameters[0] == 0 {
                    instruction_pointer = parameters[1] as usize;
                    continue 'main;
                }
            }
            LESS_THAN => {
                memory[parameters[2] as usize] = if parameters[0] < parameters[1] { 1 } else { 0 };
            }
            EQUALS => {
                memory[parameters[2] as usize] = if parameters[0] == parameters[1] { 1 } else { 0 };
            }
            HALT => break,
            _ => panic!("An unknown opcode was encountered"),
        }

        instruction_pointer += parameters.len() + 1;
    }

    output
}

fn get_parameter_count(opcode: u32) -> usize {
    match opcode {
        ADD => 3,
        MULTIPLY => 3,
        INPUT => 1,
        OUTPUT => 1,
        JUMP_IF_TRUE => 2,
        JUMP_IF_FALSE => 2,
        LESS_THAN => 3,
        EQUALS => 3,
        HALT => 0,
        _ => panic!("An unknown opcode was encountered"),
    }
}

fn location_parameter(opcode: u32, parameter_index: usize) -> bool {
    match opcode {
        ADD => parameter_index == 2,
        MULTIPLY => parameter_index == 2,
        INPUT => parameter_index == 0,
        LESS_THAN => parameter_index == 2,
        EQUALS => parameter_index == 2,
        _ => false,
    }
}
