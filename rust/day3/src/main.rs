use std::fs::File;
use std::io::Result;
use std::io::Read;
use std::collections::HashMap;

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn increment_key(position: (i32, i32), matrix: &mut HashMap<(i32, i32), i32>) -> () {
	let existing = matrix.insert(position, 1);
	if existing.is_some() {
		*matrix.get_mut(&position).unwrap() = existing.unwrap() + 1;
	}
}

fn build_matrix(input: &str) -> HashMap<(i32, i32), i32> {
	let mut matrix = HashMap::new();
	let mut position = (0, 0);
	increment_key(position, &mut matrix);

	for c in input.chars() {
		let new_position = match c {
			'^' => Some((position.0 - 1, position.1)),
			'v' => Some((position.0 + 1, position.1)),
			'>' => Some((position.0, position.1 + 1)),
			'<' => Some((position.0, position.1 - 1)),
			_ => None,
		};

		if new_position.is_some() {
			position = new_position.unwrap();
			increment_key(position, &mut matrix);
		}
	}

	return matrix;
}

fn main() {
    println!("Advent of Code - day 3");

	let input = read_file("input.txt").unwrap();
	let matrix = build_matrix(&input);
	println!("Answer: {}", matrix.len());
}
