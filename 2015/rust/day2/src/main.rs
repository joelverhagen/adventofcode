use std::cmp::min;
use std::fs::File;
use std::io::Result;
use std::io::Read;

struct Dimensions {
	l: i32,
	w: i32,
	h: i32,
}

fn parse_dimensions(input: &str) -> Vec<Dimensions> {
	let mut all: Vec<Dimensions> = Vec::new();
	for line in input.lines() {
		
		let dimensions: Vec<i32> = line
			.split('x')
			.map(|d| d.parse::<i32>().unwrap())
			.collect();
		let l = dimensions[0];
		let w = dimensions[1];
		let h = dimensions[2];

		all.push(Dimensions { l: l, w: w, h: h});
	}

	all
}

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));

    Ok(content)
}

fn get_total_square_feet(dimensions: &Vec<Dimensions>) -> i32 {
	let mut total = 0;
	for d in dimensions {
		let side_a = d.l*d.w;
		let side_b = d.w*d.h;
		let side_c = d.h*d.l;
		let min_side = min(min(side_a, side_b), side_c);

		total += (2*side_a) + (2*side_b) + (2*side_c) + min_side;
	}

	total
}

fn get_total_length(dimensions: &Vec<Dimensions>) -> i32 {
	let mut total = 0;
	for d in dimensions {
		let perim_a = 2*d.l+2*d.w;
		let perim_b = 2*d.w+2*d.h;
		let perim_c = 2*d.h+2*d.l;
		let min_perim = min(min(perim_a, perim_b), perim_c);

		let volume = d.l*d.w*d.h;

		total += volume + min_perim;
	}

	total
}

fn main() {
    println!("Advent of Code - day 2");

	let input = read_file("input.txt").unwrap();
	let dimensions = parse_dimensions(&input);
	println!("Part 1 answer: {}", get_total_square_feet(&dimensions));
	println!("Part 2 answer: {}", get_total_length(&dimensions));
}
