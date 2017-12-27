extern crate adventofcode;

use std::fs::File;
use std::io::prelude::*;
use std::fmt::Write;
use adventofcode::day10::{knot_hash, evaluate_lengths};

fn read_string(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("Could not open the specified file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read the file.");

    contents
}

fn read_lengths(file_name: &str) -> Vec<u32> {
    read_string(file_name)
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|&p| p.len() > 0)
        .map(|p| p.parse::<u32>().expect("Could not parse a length as u32."))
        .collect()
}

fn read_list(file_name: &str) -> Vec<u32> {
    read_string(file_name)
        .chars()
        .map(|c| c as u32)
        .collect()
}

fn get_product_of_first_two(list_size: u32, lengths: &Vec<u32>) -> u32 {
    let mut list = (0..list_size).map(|x| x as u8).collect();
    evaluate_lengths(&mut list, lengths, 0, 0);
    
    list[0] as u32 * list[1] as u32
}

fn knot_hash_hex(input: &Vec<u32>) -> String {
    let dense_hash = knot_hash(input);
    
    let mut hash = String::new();
    for b in dense_hash {
        write!(&mut hash, "{:02x}", b).expect("Unable to write hex.");
    }

    hash
}

fn main() {
    let list_size = 256;
    let file_name = "input.txt";
    println!("Day 10, part 1: {}", get_product_of_first_two(list_size, &read_lengths(file_name)));
    println!("Day 10, part 2: {}", knot_hash_hex(&read_list(file_name)));
}
