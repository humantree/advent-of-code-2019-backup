use std::fs;

mod intcode;

fn main() {
    let memory: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Unable to open input.txt")
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    intcode::run(memory);
}
