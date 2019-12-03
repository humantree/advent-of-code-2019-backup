use std::fs;

fn main() {
    let mut total = 0;
    let input = fs::read_to_string("input.txt").expect("Unable to open input.txt");

    for line in input.lines() {
        let module_mass = line.parse::<isize>().expect("Mass was not an integer");
        total += calculate_module_fuel(module_mass);
    }

    println!("Total fuel required: {}", total);
}

fn calculate_fuel(mass: isize) -> isize {
    mass / 3 - 2
}

fn calculate_module_fuel(mass: isize) -> isize {
    let mut fuel = calculate_fuel(mass);
    let mut new_mass = fuel;

    loop {
        let extra_fuel = calculate_fuel(new_mass);

        if extra_fuel <= 0 {
            break;
        }

        fuel += extra_fuel;
        new_mass = extra_fuel;
    }

    fuel
}
