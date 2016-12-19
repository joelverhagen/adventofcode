use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum CommandTypeParseError {
    CouldNotReadFile,
    WrongNumberOfPieces,
    CouldNotParseNumber,
    UnknownCommand,
}

#[derive(Debug)]
enum CommandType {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

#[derive(Debug)]
struct Display {
    pixels: Vec<Vec<bool>>
}

impl Display {
    fn new(width: usize, height: usize) -> Display {
        let pixels = vec![vec![false; width]; height];
        Display {
            pixels: pixels,
        }
    }

    fn pretty_print(&self) -> String {
        let mut output = String::new();
        for row in &self.pixels {
            for pixel in row {
                output.push(match *pixel {
                    false => '.',
                    true  => '#',
                });
            }

            output.push('\n');
        }

        output
    }

    fn execute_command(&mut self, command_type: &CommandType) {
        match command_type {
            &CommandType::Rect(width, height)         => self.execute_rect(width, height),
            &CommandType::RotateColumn(column, count) => self.execute_rotate_column(column, count),
            &CommandType::RotateRow(row, count)       => self.execute_rotate_row(row, count),
        };
    }

    fn execute_commands(&mut self, command_types: &Vec<CommandType>) {
        for command_type in command_types {
            self.execute_command(command_type);
        }
    }

    fn execute_rect(&mut self, width: usize, height: usize) {
        for column in 0..width {
            for row in 0..height {
                self.pixels[row][column] = true;
            }
        }
    }

    fn get_row(&self, row: usize) -> Vec<bool> {
        self.pixels[row].clone()
    }

    fn get_column(&self, column: usize) -> Vec<bool> {
        let mut output: Vec<bool> = Vec::new();
        for row in 0..self.pixels.len() {
            output.push(self.pixels[row][column]);
        }

        output
    }

    fn execute_rotate_column(&mut self, column: usize, count: usize) {
        let previous_column = self.get_column(column);
        for old_row in 0..previous_column.len() {
            let new_row = (old_row + count) % previous_column.len();
            self.pixels[new_row][column] = previous_column[old_row];
        }
    }

    fn execute_rotate_row(&mut self, row: usize, count: usize) {
        let previous_row = self.get_row(row);
        for old_column in 0..previous_row.len() {
            let new_column = (old_column + count) % previous_row.len();
            self.pixels[row][new_column] = previous_row[old_column];
        }
    }

    fn count_on_pixels(&self) -> i32 {
        let mut count = 0;
        for row in &self.pixels {
            for pixel in row {
                if *pixel {
                    count += 1;
                }
            }
        }

        count
    }
}

impl CommandType {
    fn parse(unparsed: &str) -> Result<CommandType, CommandTypeParseError> {
        let pieces: Vec<&str> = unparsed.split(|c| c == ' ' || c == '=').collect();

        if pieces.len() < 2 {
            return Err(CommandTypeParseError::WrongNumberOfPieces)
        }

        match pieces[0] {
            "rect"   => CommandType::parse_rect(&pieces[1..]),
            "rotate" => CommandType::parse_rotate(&pieces[1..]),
            _        => Err(CommandTypeParseError::UnknownCommand),
        }
    }

    fn parse_rect(p: &[&str]) -> Result<CommandType, CommandTypeParseError> {
        if p.len() != 1 {
            return Err(CommandTypeParseError::WrongNumberOfPieces)
        }

        let p: Vec<&str> = p[0].split('x').collect();
        if p.len() != 2 {
            return Err(CommandTypeParseError::WrongNumberOfPieces)
        }

        let pair = CommandType::parse_usize_pair(p[0], p[1])?;

        Ok(CommandType::Rect(pair.0, pair.1))
    }

    fn parse_rotate(p: &[&str]) -> Result<CommandType, CommandTypeParseError> {
        if p.len() != 5 {
            return Err(CommandTypeParseError::WrongNumberOfPieces)
        }

        match (p[0], p[1], p[2], p[3], p[4]) {
            ("column", _, a, "by", b) => {
                let pair = CommandType::parse_usize_pair(a, b)?;
                Ok(CommandType::RotateColumn(pair.0, pair.1))
            },
            ("row"   , "y", a, "by", b) => {
                let pair = CommandType::parse_usize_pair(a, b)?;
                Ok(CommandType::RotateRow(pair.0, pair.1))
            },
            _                           => {
                Err(CommandTypeParseError::UnknownCommand)
            }
        }
    }

    fn parse_usize_pair(a: &str, b: &str) -> Result<(usize, usize), CommandTypeParseError> {
        let a = CommandType::parse_usize(a)?;
        let b = CommandType::parse_usize(b)?;

        Ok((a, b))
    }

    fn parse_usize(input: &str) -> Result<usize, CommandTypeParseError> {
        match input.parse::<usize>() {
            Ok(r)  => Ok(r),
            Err(_) => return Err(CommandTypeParseError::CouldNotParseNumber),
        }
    }
}

fn parse_command_types(path: &str) -> Result<Vec<CommandType>, CommandTypeParseError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_)   => return Err(CommandTypeParseError::CouldNotReadFile),
    };

    let file_reader = BufReader::new(file);
    let mut command_types: Vec<CommandType> = Vec::new();

    for line_result in file_reader.lines() {
        let command_type = match line_result {
            Ok(line) => CommandType::parse(&line)?,
            Err(_)   => return Err(CommandTypeParseError::CouldNotReadFile),
        };

        command_types.push(command_type);
    }

    Ok(command_types)
}

fn main() {
    let command_types = parse_command_types("input.txt").unwrap();
    let mut display = Display::new(50, 6);
    display.execute_commands(&command_types);

    let part_1_result = display.count_on_pixels();
    println!("Part 1 result: {}", part_1_result);
}
