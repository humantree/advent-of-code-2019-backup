use std::fs;

mod intcode;

fn main() {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut memory: Vec<usize> = fs::read_to_string("input.txt")
                .expect("Unable to open input.txt")
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect();

            memory[1] = i;
            memory[2] = j;

            let results = intcode::run(memory);
            if results[0] == 19690720 {
                println!("{}", 100 * i + j);
                return;
            }
        }
    }
}
