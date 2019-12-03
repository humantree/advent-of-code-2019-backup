pub fn run(mut memory: Vec<usize>) -> Vec<usize> {
    let mut instruction_pointer = 0;

    loop {
        let opcode = memory[instruction_pointer];

        match opcode {
            1 => {
                let first = memory[memory[instruction_pointer + 1]];
                let second = memory[memory[instruction_pointer + 2]];
                let location = memory[instruction_pointer + 3];
                memory[location] = first + second;
            }
            2 => {
                let first = memory[memory[instruction_pointer + 1]];
                let second = memory[memory[instruction_pointer + 2]];
                let location = memory[instruction_pointer + 3];
                memory[location] = first * second;
            }
            99 => break,
            _ => panic!("An unknown opcode was encountered"),
        }

        instruction_pointer += 4;
    }

    memory
}
