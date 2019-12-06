use std::fs::read_to_string;
use std::iter::successors;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let modules: Vec<u32> = input.lines().map(parse_module).collect();
    let fuel_required: u32 = modules.iter().map(calculate_module_fuel).sum();
    println!("Total fuel required: {}", fuel_required);
}

fn calculate_fuel(mass: &u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

fn calculate_module_fuel(mass: &u32) -> u32 {
    successors(Some(*mass), calculate_fuel).skip(1).sum()
}

fn parse_module(line: &str) -> u32 {
    line.parse().expect("Line was not an integer")
}

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_module_fuel};

    #[test]
    fn test_1() {
        assert_eq!(calculate_fuel(&12), Some(2));
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate_fuel(&14), Some(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(calculate_fuel(&1969), Some(654));
    }

    #[test]
    fn test_4() {
        assert_eq!(calculate_fuel(&100756), Some(33583));
    }

    #[test]
    fn test_5() {
        assert_eq!(calculate_module_fuel(&14), 2);
    }

    #[test]
    fn test_6() {
        assert_eq!(calculate_module_fuel(&1969), 966);
    }

    #[test]
    fn test_7() {
        assert_eq!(calculate_module_fuel(&100756), 50346);
    }
}
