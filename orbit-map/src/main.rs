use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::successors;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input.txt");
    let mut orbit_map = HashMap::new();

    for line in input.lines() {
        add_to_map(line, &mut orbit_map);
    }

    let total_orbits: u32 = orbit_map
        .keys()
        .map(|key| calculate_orbits(&orbit_map, key))
        .sum();

    println!("Total orbits: {}", total_orbits);

    let my_orbits = get_orbits(&orbit_map, "YOU");
    let santas_orbits = get_orbits(&orbit_map, "SAN");

    let mut closest_intersection = "COM";

    for object in &my_orbits {
        if santas_orbits.contains(&object) {
            closest_intersection = object;
            break;
        }
    }

    let minimum_orbital_transfers = find_index(my_orbits, closest_intersection)
        + find_index(santas_orbits, closest_intersection);

    println!("Minimum orbital transfers: {}", minimum_orbital_transfers);
}

fn add_to_map<'a>(line: &'a str, orbit_map: &mut HashMap<&'a str, &'a str>) {
    let objects: Vec<&str> = line.split(')').collect();
    orbit_map.insert(objects[1], objects[0]);
}

fn calculate_orbits(orbit_map: &HashMap<&str, &str>, object: &str) -> u32 {
    successors(Some(&object), |key| orbit_map.get(*key)).count() as u32 - 1
}

fn find_index(vec: Vec<&str>, item: &str) -> usize {
    vec.iter().position(|p| p == &item).unwrap()
}

fn get_orbits<'a>(orbit_map: &HashMap<&'a str, &'a str>, object: &'a str) -> Vec<&'a str> {
    successors(Some(&object), |key| orbit_map.get(*key))
        .skip(1)
        .map(|object| *object)
        .collect()
}
