use std::fs::File;
use std::io;
use std::io::Read;
use std::result::Result;
use std::cmp::min;
use std::cmp::max;
use std::fmt;
use std::fmt::Display;

extern crate bit_vec;
use bit_vec::BitVec;


fn read_file(path: &str) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

enum InstructionType {
	TurnOn,
	Toggle,
	TurnOff
}

struct Instruction {
	instruction_type: InstructionType,
	upper_left: (i32, i32),
	lower_right: (i32, i32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let instruction_type = match self.instruction_type {
    		InstructionType::TurnOn => "turn on ",
    		InstructionType::Toggle => "toggle ",
    		InstructionType::TurnOff => "turn off "
    	};

        return write!(
        	f,
        	"{}{},{} through {},{}",
        	instruction_type,
        	self.upper_left.0,
        	self.upper_left.1,
        	self.lower_right.0,
        	self.lower_right.1);
    }
}

impl Instruction {
	fn parse(input: &str) -> Result<Instruction, &str> {
		let mut remaining = input.clone();

		// parse out the instruction type
		let instruction_type;
		if remaining.starts_with("turn on ") {
			instruction_type = InstructionType::TurnOn;
			remaining = &remaining[8..];
		} else if remaining.starts_with("toggle ") {
			instruction_type = InstructionType::Toggle;
			remaining = &remaining[7..];
		} else if remaining.starts_with("turn off ") {
			instruction_type = InstructionType::TurnOff;
			remaining = &remaining[9..];
		} else {
			return Err("The instruction did not start with a valid type.");
		}

		// parse out the remaining pieces
		let pieces: Vec<&str> = remaining.split(' ').collect();
		if pieces.len() != 3 || pieces[1] != "through" {
			return Err("There should be three space-separated pieces after the instruction type.");
		}

		// parse the coordinates
		let coordinate_a_option = Instruction::parse_coordinate(pieces[0]);
		if coordinate_a_option.is_err() {
			return Err(coordinate_a_option.err().unwrap());
		}
		let coordinate_b_option = Instruction::parse_coordinate(pieces[2]);
		if coordinate_b_option.is_err() {
			return Err(coordinate_b_option.err().unwrap());
		}

		// make sure we have upper left and lower right
		let upper_left = (
			min(coordinate_a_option.unwrap().0, coordinate_b_option.unwrap().0),
			min(coordinate_a_option.unwrap().1, coordinate_b_option.unwrap().1)
		);

		let lower_right = (
			max(coordinate_a_option.unwrap().0, coordinate_b_option.unwrap().0),
			max(coordinate_a_option.unwrap().1, coordinate_b_option.unwrap().1)
		);

		return Ok(Instruction {
			instruction_type: instruction_type,
			upper_left: upper_left,
			lower_right: lower_right
		});
	}

	fn parse_coordinate(input: &str) -> Result<(i32, i32), &str> {
		let pieces: Vec<&str> = input.split(',').collect();
		if pieces.len() != 2 {
			return Err("There should be one comma in a coordinate.");
		}

		let x_option = pieces[0].parse::<i32>();
		let y_option = pieces[1].parse::<i32>();

		if x_option.is_err() || y_option.is_err() {
			return Err("The x and y coordinate could not be parsed as integers.");
		}

		return Ok((x_option.unwrap(), y_option.unwrap()));
	}
}

fn print_grid(grid: &BitVec, width: usize, height: usize) -> () {
	for x in 0..width {
		for y in 0..height {
			let index = y * width + x;
			if grid.get(index).unwrap() {
				print!("1");
			} else {
				print!("0");
			}
		}
		println!("");
	}

	println!("");
}

fn process_instructions(input: &str, width: usize, height: usize) -> i32 {
	// initialize the grid
	let mut grid = BitVec::from_elem(width * height, false);

	// process the instructions
	for line in input.lines() {
		let instruction_option = Instruction::parse(line);
		if instruction_option.is_err() {
			continue;
		}

		let instruction = instruction_option.unwrap();

		for x in instruction.upper_left.0..instruction.lower_right.0 + 1 {
			for y in instruction.upper_left.1..instruction.lower_right.1 + 1 {
				let index = ((y as usize) * width) + (x as usize);
				let old_value = grid.get(index).unwrap();
				let new_value = match instruction.instruction_type {
					InstructionType::TurnOn => true,
					InstructionType::Toggle => !old_value,
					InstructionType::TurnOff => false,
				};

				grid.set(index as usize, new_value);
			}
		}
	}

	// count true 
	let mut count = 0;
	for value in grid {
		if value {
			count += 1;
		}
	}

	return count;
}

fn main() {
    println!("Advent of Code - day 6");

	let input = read_file("input.txt").unwrap();
	let answer = process_instructions(&input, 1000, 1000);
	println!("Answer: {}", answer);
	
	// let input = "turn on 0,0 through 3,3\nturn off 1,1 through 2,2\ntoggle 0,0 through 3,3";
	// let answer = process_instructions(&input, 4, 4);
	// println!("Answer: {}", answer);
}
