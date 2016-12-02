use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::result::Result;

#[derive(Debug)]
enum Direction {
    Right,
    Left
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: i32,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction_char = match self.direction {
            Direction::Right => 'R',
            Direction::Left  => 'L',
        };

        write!(f, "{}{}", direction_char, self.distance)
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_steps(unparsed_steps: &str) -> Result<Vec<Step>, String> {
    let split_steps: Vec<&str> = unparsed_steps.split(",").collect();
    let mut parsed_steps: Vec<Step> = Vec::with_capacity(split_steps.len());

    for unparsed_step in split_steps {
        let trimmed_step = unparsed_step.trim();
        let step_result = parse_step(&trimmed_step);

        if step_result.is_err() {
            return Err(step_result.err().unwrap());
        }

        parsed_steps.push(step_result.unwrap());
    }

    Ok(parsed_steps)
}

fn parse_step(unparsed_step: &str) -> Result<Step, String> {
    if unparsed_step.len() < 2 {
        return Err("The step string must have at least 2 characters.".to_string());
    }

    let first_char = unparsed_step.chars().nth(0).unwrap();
    let direction;
    match first_char {
        'R' => direction = Direction::Right,
        'L' => direction = Direction::Left,
        _   => return Err("The first character of the step must be R or L.".to_string()),
    }

    let number_part = &unparsed_step[1..];
    let number_result = number_part.parse::<i32>();
    if number_result.is_err() {
        return Err(format!("The number part {} of the step could not be parsed to an integer.", number_part));
    }

    Ok(Step {
        direction: direction,
        distance: number_result.unwrap(),
    })
}

fn evaluate_steps(steps: &Vec<Step>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;

    println!("initial -> x: {}, y: {}, direction: {}", x, y, direction);

    for step in steps {
        let direction_delta = match step.direction {
            Direction::Right => -1,
            Direction::Left  => 1,
        };

        direction = (direction + direction_delta + 4) % 4;
        println!("{} -> x: {}, y: {}, direction: {}", step, x, y, direction);

        match direction {
            0 => y += step.distance,
            1 => x += step.distance,
            2 => y -= step.distance,
            3 => x -= step.distance,
            _ => {},
        }
    }

    x.abs() + y.abs()
}

fn main() {
    let input = read_file("input.txt").unwrap();
    let steps = parse_steps(&input).unwrap();
    let distance = evaluate_steps(&steps);
    println!("Result: {}", distance);
}
