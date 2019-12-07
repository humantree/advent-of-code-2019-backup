mod amplifier;
mod intcode;

fn main() {
    let mut maximum_thruster_signal = 0;
    let mut values = vec![0, 1, 2, 3, 4];
    let mut combinations = Vec::<Vec<i32>>::new();
    generate_combinations(values.len(), &mut values, &mut combinations);

    for combination in combinations {
        let a = amplifier::run(combination[0], 0);
        let b = amplifier::run(combination[1], a);
        let c = amplifier::run(combination[2], b);
        let d = amplifier::run(combination[3], c);
        let e = amplifier::run(combination[4], d);

        if e > maximum_thruster_signal {
            maximum_thruster_signal = e;
        }
    }

    println!("Max thruster signal: {}", maximum_thruster_signal);
}

fn generate_combinations(i: usize, values: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>) {
    if i == 1 {
        combinations.push(values.to_vec());
        return;
    }

    for j in 0..i - 1 {
        generate_combinations(i - 1, values, combinations);
        values.swap(i - 1, if i % 2 == 0 { j } else { 0 });
    }

    generate_combinations(i - 1, values, combinations);
}
