use std::cmp::min;
use std::fs::File;
use std::io::Result;
use std::io::Read;

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn get_square_feet(l: i32, w: i32, h: i32) -> i32 {
	let side_a = l*w;
	let side_b = w*h;
	let side_c = h*l;
	let min_side = min(min(side_a, side_b), side_c);

	return (2*side_a) + (2*side_b) + (2*side_c) + min_side;
}

fn get_total_square_feet(input: &str) -> i32 {
	let mut total = 0;
	for line in input.lines() {
		
		let dimensions: Vec<i32> = line
			.split('x')
			.map(|d| d.parse::<i32>().unwrap())
			.collect();
		let l = dimensions[0];
		let w = dimensions[1];
		let h = dimensions[2];

		total += get_square_feet(l, w, h);
	}

	return total;
}

fn main() {
    println!("Advent of Code - day 2");

	let input = read_file("input.txt").unwrap();
	let total = get_total_square_feet(&input);
	println!("Answer: {}", total);
}
