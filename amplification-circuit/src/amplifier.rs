use crate::intcode;
use std::fs;

pub fn run(phase: i32, input: i32) -> i32 {
  let memory: Vec<i32> = fs::read_to_string("input.txt")
    .expect("Unable to open input.txt")
    .split(",")
    .map(|s| s.trim().parse().unwrap())
    .collect();

  let inputs = vec![input, phase];

  intcode::run(memory, inputs)
}
