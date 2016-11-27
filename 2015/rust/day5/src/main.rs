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

fn has_repeated_pair(input: &str) -> bool {
	let mut chars = input.chars();
	let mut last_letter = chars.next().unwrap();

	for c in chars {
		let pair: String = vec![last_letter, c].into_iter().collect();
		last_letter = c;

		let first = input.find(&pair[..]);
		if first.is_none() {
			continue;
		}

		let second = input[first.unwrap() + 2..].find(&pair[..]);
		if second.is_none() {
			continue;
		}

		return true;
	}

	return false;
}

fn has_sandwiched_char(input: &str) -> bool {
	let chars: Vec<char> = input.chars().collect();
	for i in 0..chars.len() - 2 {
		if chars[i] == chars[i + 2] {
			return true;
		}
	}

	return false;
}

fn has_no_naughty(input: &str) -> bool {
	return !input.contains("ab") &&
	       !input.contains("cd") &&
	       !input.contains("pq") &&
	       !input.contains("xy");
}

fn is_nice_part_1(input: &str) -> bool {
	return has_3_vowels(input) &&
	       has_double_letter(input) &&
	       has_no_naughty(input);
}

fn is_nice_part_2(input: &str) -> bool {
	return has_repeated_pair(input) &&
	       has_sandwiched_char(input);
}

fn count_nice_part_1(input: &str) -> i32 {
	let mut count = 0;
	for line in input.lines() {
		if is_nice_part_1(line) {
			count += 1;
		}
	}

	return count;
}

fn count_nice_part_2(input: &str) -> i32 {
	let mut count = 0;
	for line in input.lines() {
		if is_nice_part_2(line) {
			count += 1;
		}
	}

	return count;
}

fn main() {
    println!("Advent of Code - day 5");

	let input = read_file("input.txt").unwrap();
	println!("Part 1 answer: {}", count_nice_part_1(&input));
	println!("Part 2 answer: {}", count_nice_part_2(&input));
}
