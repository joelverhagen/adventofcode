use std::fs::File;
use std::io::prelude::*;
use std::fmt::Write;

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

fn evaluate_lengths(list: &mut Vec<u32>, lengths: &Vec<u32>, initial_position: u32, initial_skip_size: u32) -> (u32, u32) {
    let list_size = list.len() as u32;
    let mut position = initial_position;
    let mut skip_size = initial_skip_size;

    for length in lengths {
        if *length > list_size {
            continue;
        }

        for i in 0..length / 2 {
            let from_index = ((position + i) % list_size) as usize;
            let to_index = ((position + (length - i) - 1) % list_size) as usize;
            let temp = list[to_index];
            list[to_index] = list[from_index];
            list[from_index] = temp;
        }

        position = (position + length + skip_size) % list_size;
        skip_size += 1;
    }

    (position, skip_size)
}

fn get_product_of_first_two(list_size: u32, lengths: &Vec<u32>) -> u32 {
    let mut list = (0..list_size).collect();
    evaluate_lengths(&mut list, lengths, 0, 0);
    
    list[0] * list[1]
}

fn knot_hash(input: &Vec<u32>) -> String {
    let mut sparse_hash: Vec<u32> = (0..256).collect();
    let mut lengths = input.clone();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        let (next_position, next_skip_size) = evaluate_lengths(&mut sparse_hash, &lengths, position, skip_size);
        position = next_position;
        skip_size = next_skip_size;
    }

    let mut dense_hash = Vec::new();
    for i in 0..sparse_hash.len() / 16 {
        let starting_index = i * 16;
        let mut current = sparse_hash[starting_index];
        for offset in 1..16 {
            let index = starting_index + offset;
            current ^= sparse_hash[index];
        }

        dense_hash.push(current);
    }
    
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
    println!("Day 10, part 2: {}", knot_hash(&read_list(file_name)));
}
