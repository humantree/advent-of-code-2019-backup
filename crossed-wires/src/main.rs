use std::fs;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input.txt");
    let first_wire = input.lines().nth(0).unwrap().split(",").collect();
    let second_wire = input.lines().nth(1).unwrap().split(",").collect();

    let first_points = calculate_points(first_wire);
    let second_points = calculate_points(second_wire);

    let intersections: Vec<&Point> = first_points
        .iter()
        .filter(|point| **point != Point { x: 0, y: 0 })
        .filter(|point| second_points.contains(point))
        .collect();

    let intersection_distances: Vec<i32> = intersections
        .iter()
        .map(|intersection| intersection.x.abs() + intersection.y.abs())
        .collect();

    let mut closest_intersection = intersection_distances[0];
    for distance in intersection_distances {
        if distance < closest_intersection {
            closest_intersection = distance
        };
    }

    println!(
        "Distance to the closest intersection: {}",
        closest_intersection
    );

    let intersection_steps: Vec<usize> = intersections
        .iter()
        .map(|&intersection| {
            let first_steps = first_points
                .iter()
                .position(|point| point == intersection)
                .unwrap();

            let second_steps = second_points
                .iter()
                .position(|point| point == intersection)
                .unwrap();

            first_steps + second_steps
        })
        .collect();

    let mut fewest_steps = intersection_steps[0];
    for steps in intersection_steps {
        if steps < fewest_steps {
            fewest_steps = steps;
        };
    }

    println!("Steps to the closest intersection: {:?}", fewest_steps);
}

fn add_points(command: &str, points: &mut Vec<Point>) {
    let direction = &command[0..1];
    let distance: i32 = command[1..]
        .parse()
        .expect("Non-numeric distance was found");

    for _ in 0..distance {
        let mut x = points.last().unwrap().x;
        let mut y = points.last().unwrap().y;

        match direction {
            "U" => y += 1,
            "D" => y -= 1,
            "L" => x -= 1,
            "R" => x += 1,
            _ => panic!("Unexpected direction found"),
        }

        &points.push(Point { x, y });
    }
}

fn calculate_points(commands: Vec<&str>) -> Vec<Point> {
    let mut points = vec![Point { x: 0, y: 0 }];

    for command in commands.iter() {
        add_points(command, &mut points);
    }

    points
}
