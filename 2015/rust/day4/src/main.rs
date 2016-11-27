use std::thread;
use std::sync::mpsc;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

struct Input {
	start: i32,
	increment: i32,
	key: String,
	prefix: String,
}


fn find_answer(key: &str, prefix: &str) -> i32 {
	let increment = 8;
	let mut children = Vec::new();
	let (tx, rx) = mpsc::channel();
	
	for start in 0..increment {
		let tx = tx.clone();
		let input = Input { start: start, increment: increment, key: key.to_string(), prefix: prefix.to_string() };
		let child = thread::spawn(move || {
			let mut current = input.start;
			let mut md5 = Md5::new();
			loop {
				let hash_input = format!("{}{}", input.key, current);
				md5.input_str(&hash_input);
				let hash_output = md5.result_str();
				md5.reset();

				if hash_output.starts_with(&input.prefix) {
					tx.send(current);
				} else {
					current += input.increment;
				}
			}
		});

		children.push(child);
	}

	return rx.recv().unwrap();
}


fn main() {
	println!("Advent of Code - day 4");

	let input = "yzbqklnj";
    println!("Secret key: {}", input);
    // println!("Part 1 answer: {}", find_answer(input, "00000"));
    println!("Part 2 answer: {}", find_answer(input, "000000"));
}
