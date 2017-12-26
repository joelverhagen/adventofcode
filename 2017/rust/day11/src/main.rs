use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

fn parse_directions(path: &str) -> Vec<Direction> {
    let mut f = File::open(path).expect("Could not open the specified file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could read the file contents.");

    let mut directions = Vec::new();
    for p in contents.split(',') {
        let direction = match p {
            "n"  => Direction::North,
            "ne" => Direction::Northeast,
            "se" => Direction::Southeast,
            "s"  => Direction::South,
            "sw" => Direction::Southwest,
            "nw" => Direction::Northwest,
            _    => panic!("Unexpected direction."),
        };

        directions.push(direction);
    }

    directions
}

fn process_directions(directions: &Vec<Direction>) -> ((i32, i32, i32), i32) { 
    // Use the cube coordinate system described here:
    // https://www.redblobgames.com/grids/hexagons/
    //
    // We maintain the following invariant:
    // x + y + z = 0

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut max_distance = 0;

    for direction in directions {
        match *direction {
            Direction::North     => { y += 1; z -= 1; },
            Direction::Northeast => { x += 1; z -= 1; },
            Direction::Southeast => { x += 1; y -= 1; },
            Direction::South     => { y -= 1; z += 1; },
            Direction::Southwest => { x -= 1; z += 1; },
            Direction::Northwest => { x -= 1; y += 1; },
        }

        let distance = get_distance((x, y, z));
        if distance > max_distance {
            max_distance = distance
        }
    }

    ((x, y, z), max_distance)
}

fn get_distance(location: (i32, i32, i32)) -> i32 {
    (location.0.abs() + location.1.abs() + location.2.abs()) / 2
}

fn main() {
    let path = "input.txt";
    let directions = parse_directions(&path);
    println!("Day 11, part 1: {}", get_distance(process_directions(&directions).0));
    println!("Day 11, part 2: {}", process_directions(&directions).1);
}
