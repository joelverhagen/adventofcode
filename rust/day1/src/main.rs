use std::fs::File;
use std::io::Result;
use std::io::Read;

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn get_final_floor(input: &str) -> i32 {
	let mut current_floor = 0;
	for c in input.chars() {
		let delta = match c {
			'(' => 1,
			')' => -1,
            _ => 0
		};
		current_floor += delta;
	}

	return current_floor;
}

fn main() {
	println!("Advent of Code - day 1");

	let input = read_file("input.txt").unwrap();
	let final_floor = get_final_floor(&input);
	println!("Answer: {}", final_floor);
}
