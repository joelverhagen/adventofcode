extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn find_answer(key: &str) -> i32 {
	let mut current = 0;
	let mut md5 = Md5::new();

	loop {
		let hash_input = format!("{}{}", key, current);
		md5.input_str(&hash_input);
		let hash_output = md5.result_str();
		md5.reset();

		if hash_output.starts_with("00000") {
			break;
		} else {
			current += 1;
		}
	}

	return current;
}


fn main() {
	println!("Advent of Code - day 4");

	let input = "yzbqklnj";
    println!("Secret key: {}", input);
    println!("Answer: {}", find_answer(input));
}
