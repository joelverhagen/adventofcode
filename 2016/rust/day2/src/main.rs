use std::fs::File;
use std::io::prelude::Read;
use std::result::Result;

static KEYPAD: &'static [[i32; 3]; 3] =
    &[[1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]];
static INITIAL_ROW: usize = 1;
static INITIAL_COL: usize = 1;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct KeypadPosition {
    row: usize,
    col: usize
}

fn read_file_with_io_error(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file(path: &str) -> Result<String, String> {
    match read_file_with_io_error(path) {
        Ok(output) => Ok(output),
        Err(err)   => Err(format!("{}", err)),
    }
}

fn parse_file(input: &str) -> Result<Vec<Vec<Direction>>, String> {
    let mut output: Vec<Vec<Direction>> = Vec::new();

    for unparsed_line in input.lines() {
        let trimmed_line = unparsed_line.trim_right();
        let line = parse_line(trimmed_line)?;
        output.push(line);
    }

    Ok(output)
}

fn parse_line(unparsed_line: &str) -> Result<Vec<Direction>, String> {
    let mut directions: Vec<Direction> = Vec::new();

    for unparsed_direction in unparsed_line.chars() {
        let direction = parse_direction(unparsed_direction)?;
        directions.push(direction);
    }

    Ok(directions)
}

fn parse_direction(unparsed_direction: char) -> Result<Direction, String> {
    match unparsed_direction {
        'U' => Ok(Direction::Up),
        'D' => Ok(Direction::Down),
        'L' => Ok(Direction::Left),
        'R' => Ok(Direction::Right),
        _   => Err(format!("The direction character {} is not a valid direction.", unparsed_direction))
    }
}

fn evaluate_line(initial: &KeypadPosition, directions: &Vec<Direction>) -> KeypadPosition {
    let mut row = initial.row;
    let mut col = initial.col;
    // println!("initial -> {}, {} ({})", row, col, KEYPAD[row][col]);

    for direction in directions {
        match *direction {
            Direction::Up    if row > 0                   => row -= 1,
            Direction::Down  if row < KEYPAD.len() - 1    => row += 1,
            Direction::Right if col < KEYPAD[0].len() - 1 => col += 1,
            Direction::Left  if col > 0                   => col -= 1,
            _                                             => {},   
        }

        // println!("{:?} -> {}, {} ({})", direction, row, col, KEYPAD[row][col]);
    }

    KeypadPosition {
        row: row,
        col: col
    }
}

fn evaluate_file(path: &str) -> Result<String, String> {
    let content = read_file(path)?;
    let directions = parse_file(&content)?;

    let mut output = String::new();
    let mut position = KeypadPosition {
        row: INITIAL_ROW,
        col: INITIAL_COL,
    };

    for line in directions {
        let new_position = evaluate_line(&position, &line);
        let number = KEYPAD[new_position.row][new_position.col];
        output.push_str(&format!("{}", number));

        position = new_position;
    }

    Ok(output)
}

fn main() {
    let part_1_result = evaluate_file("input.txt").unwrap();
    println!("Part 1 result: {}", part_1_result);
}
