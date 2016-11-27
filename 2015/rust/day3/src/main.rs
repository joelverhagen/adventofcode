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

fn move_position(position: (i32, i32), c: char) -> Option<(i32, i32)> {
	match c {
		'^' => Some((position.0 - 1, position.1)),
		'v' => Some((position.0 + 1, position.1)),
		'>' => Some((position.0, position.1 + 1)),
		'<' => Some((position.0, position.1 - 1)),
		_ => None,
	}
}

fn build_matrix_part_1(input: &str) -> HashMap<(i32, i32), i32> {
	let mut matrix = HashMap::new();
	let mut position = (0, 0);
	increment_key(position, &mut matrix);

	for c in input.chars() {
		let new_position = move_position(position, c);

		if new_position.is_some() {
			position = new_position.unwrap();
			increment_key(position, &mut matrix);
		}
	}

	return matrix;
}


fn build_matrix_part_2(input: &str) -> HashMap<(i32, i32), i32> {
	let mut matrix = HashMap::new();
	let mut position_a = (0, 0);
	let mut position_b = (0, 0);
	increment_key(position_a, &mut matrix);
	increment_key(position_b, &mut matrix);
	let mut next_is_a = true;

	for c in input.chars() {
		let position = match next_is_a {
			true => position_a,
			false => position_b,
		};

		let new_position = move_position(position, c);

		if new_position.is_some() {

			if next_is_a {
				position_a = new_position.unwrap();
			} else {
				position_b = new_position.unwrap();
			}
			
			increment_key(new_position.unwrap(), &mut matrix);
			next_is_a = !next_is_a;
		}
	}

	return matrix;
}

fn main() {
    println!("Advent of Code - day 3");

	let input = read_file("input.txt").unwrap();
	println!("Part 1 answer: {}", build_matrix_part_1(&input).len());
	println!("Part 2 answer: {}", build_matrix_part_2(&input).len());
}
