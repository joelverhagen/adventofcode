use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_digits(file_name: &str) -> Vec<u32> {
    let mut fh = File::open(file_name).expect("Failed to open the specified file.");
    let mut contents = String::new();
    fh.read_to_string(&mut contents).expect("Failed to read the file contents.");

    let digits: Vec<u32> = contents
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits
}

fn calculate_sum(digits: &Vec<u32>, offset: usize) -> u32 {
    let mut sum = 0;
    for i in 0..digits.len() {
        let current = digits[i];
        let next = digits[(i + offset) % digits.len()];
        
        if current == next {
            sum += current;
        }
    }

    sum
}

fn main() {
    let file_name = "input.txt";
    let digits = read_digits(file_name);
    println!("Day 1, part 1: {}", calculate_sum(&digits, 1));
    println!("Day 1, part 2: {}", calculate_sum(&digits, digits.len() / 2));
}
