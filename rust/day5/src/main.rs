use std::fs::File;
use std::io::Result;
use std::io::Read;

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(path));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn has_3_vowels(input: &str) -> bool {
	let mut vowel_count = 0;
	for c in input.chars() {
		let delta = match c {
			'a' | 'e' | 'i' | 'o' | 'u' => 1,
			_ => 0,
		};
		vowel_count += delta;

		if vowel_count >= 3 {
			return true;
		}
	}

	return false;
}

fn has_double_letter(input: &str) -> bool {
	let mut last_letter = None;
	for c in input.chars() {
		if last_letter.is_some() && c == last_letter.unwrap() {
			return true;
		}

		last_letter = Some(c);
	}

	return false;
}

fn has_no_naughty(input: &str) -> bool {
	return !input.contains("ab") &&
	       !input.contains("cd") &&
	       !input.contains("pq") &&
	       !input.contains("xy");
}

fn is_nice(input: &str) -> bool {
	return has_3_vowels(input) &&
	       has_double_letter(input) &&
	       has_no_naughty(input);
}

fn count_nice(input: &str) -> i32 {
	let mut count = 0;
	for line in input.lines() {
		if is_nice(line) {
			count += 1;
		}
	}

	return count;
}

fn main() {
    println!("Advent of Code - day 5");

	let input = read_file("input.txt").unwrap();
	println!("Answer: {}", count_nice(&input));
}
