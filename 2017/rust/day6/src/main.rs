use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_memory_banks(file_name: &str) -> Vec<u16> {
    let mut f = File::open(file_name).expect("Could not open the specified file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read the file contents.");

    contents
        .split_whitespace()
        .map(|x| x.parse::<u16>().expect("Could not parse bank as u16."))
        .collect()
}

fn redistribute_until_cycle(banks: &mut Vec<u16>) -> (usize, usize) {
    let mut states = HashMap::new();
    let mut steps = 0;
    let mut first_occurrence_step = 0;

    loop {
        steps += 1;
        redistribute(banks);
        let existing = states.insert(banks.clone(), steps);
        if existing.is_some() {
            first_occurrence_step = existing.unwrap();
            break;
        }
    }

    (steps, steps - first_occurrence_step)
}

fn redistribute(banks: &mut Vec<u16>) {
    if banks.len() == 0 {
        return;
    }

    let mut max_index = 0;
    for i in 1..banks.len() {
        if banks[i] > banks[max_index] {
            max_index = i;
        }
    }

    let max_value = banks[max_index];
    banks[max_index] = 0;

    let mut index = (max_index + 1) % banks.len();    
    for _ in 0..max_value {
        banks[index] += 1;
        index = (index + 1) % banks.len();
    }
}

fn main() {
    let file_name = "input.txt";
    println!("Day 6, part 1: {}", redistribute_until_cycle(&mut read_memory_banks(file_name)).0);
    println!("Day 6, part 2: {}", redistribute_until_cycle(&mut read_memory_banks(file_name)).1);
}
