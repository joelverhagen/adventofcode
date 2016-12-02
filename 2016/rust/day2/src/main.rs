use std::fs::File;
use std::io::prelude::Read;
use std::result::Result;

struct Keypad {
    keys: Vec<Vec<char>>,
    initial_position: KeypadPosition,
}

const IGK: char = ' ';

impl Keypad {
    fn new_part_1_keypad() -> Keypad {
        Keypad {
            keys: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
            initial_position: KeypadPosition {
                row: 1,
                col: 1,
            }
        }
    }

    fn new_part_2_keypad() -> Keypad {
        Keypad {
            keys: vec![
                vec![IGK, IGK, '1', IGK, IGK],
                vec![IGK, '2', '3', '4', IGK],
                vec!['5', '6', '7', '8', '9'],
                vec![IGK, 'A', 'B', 'C', IGK],
                vec![IGK, IGK, 'D', IGK, IGK],
            ],
            initial_position: KeypadPosition {
                row: 2,
                col: 0,
            }
        }
    }

    fn is_not_ignore_key(&self, row: usize, col: usize) -> bool {
        self.key_at_coords(row, col) != IGK
    }

    fn can_go_up(&self, row: usize, col: usize) -> bool {
        row > 0 && self.is_not_ignore_key(row - 1, col)
    }

    fn can_go_down(&self, row: usize, col: usize) -> bool {
        row < &self.keys.len() - 1 && self.is_not_ignore_key(row + 1, col)
    }

    fn can_go_right(&self, row: usize, col: usize) -> bool {
        col < &self.keys[col].len() - 1 && self.is_not_ignore_key(row, col + 1)
    }

    fn can_go_left(&self, row: usize, col: usize) -> bool {
        col > 0 && self.is_not_ignore_key(row, col - 1)
    }

    fn key(&self, position: &KeypadPosition) -> char {
        self.key_at_coords(position.row, position.col)
    }

    fn key_at_coords(&self, row: usize, col: usize) -> char {
        *&self.keys[row][col]
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct KeypadPosition {
    row: usize,
    col: usize,
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

fn evaluate_line(keypad: &Keypad, initial: &KeypadPosition, directions: &Vec<Direction>) -> KeypadPosition {
    let mut row = initial.row;
    let mut col = initial.col;
    // println!("initial -> {}, {} ({})", row, col, keypad.key_at_coords(row, col));

    for direction in directions {
        match *direction {
            Direction::Up    if keypad.can_go_up(row, col)    => row -= 1,
            Direction::Down  if keypad.can_go_down(row, col)  => row += 1,
            Direction::Right if keypad.can_go_right(row, col) => col += 1,
            Direction::Left  if keypad.can_go_left(row, col)  => col -= 1,
            _                                                 => {},   
        }

        // println!("{:?} -> {}, {} ({})", direction, row, col, keypad.key_at_coords(row, col));
    }

    KeypadPosition {
        row: row,
        col: col
    }
}

fn evaluate_file(keypad: &Keypad, path: &str) -> Result<String, String> {
    let content = read_file(path)?;
    let directions = parse_file(&content)?;

    let mut output = String::new();
    let mut position = keypad.initial_position;

    for line in directions {
        let new_position = evaluate_line(&keypad, &position, &line);
        let key = keypad.key(&new_position);
        output.push(key);

        position = new_position;
    }

    Ok(output)
}

fn main() {
    let path = "input.txt";
    
    let part_1_keypad = Keypad::new_part_1_keypad();
    let part_1_result = evaluate_file(&part_1_keypad, path).unwrap();
    println!("Part 1 result: {}", part_1_result);

    let part_2_keypad = Keypad::new_part_2_keypad();
    let part_2_result = evaluate_file(&part_2_keypad, path).unwrap();
    println!("Part 2 result: {}", part_2_result);
}
