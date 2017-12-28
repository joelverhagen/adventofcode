use std::fs::File;
use std::io::{BufRead, BufReader};

struct Generator {
    previous: u64,
    factor: u64,
}

impl Generator {
    fn new_a(previous: u64) -> Generator {
        Generator {
            previous,
            factor: 16807,
        }
    }

    fn new_b(previous: u64) -> Generator {
        Generator {
            previous,
            factor: 48271,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = (self.previous * self.factor) % 2147483647;
        self.previous = next;
        Some(next)
    }
}

fn have_matching_lower_16_bits(a: u64, b: u64) -> bool {
    a & 0xffff == b & 0xffff
}

fn get_starting_number(line: &str) -> u64 {
    line
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .expect("Could not parse the starting number as a u64.")
}

fn get_starting_numbers(path: &str) -> (u64, u64) {
    let f = File::open(path).expect("Could not open the specified file.");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .map(|lr| lr.expect("Could not read a line."))
        .collect();

    (get_starting_number(&lines[0]), get_starting_number(&lines[1]))
}

fn get_part_1(starting_numbers: (u64, u64)) -> usize {
    let a = Generator::new_a(starting_numbers.0);
    let b = Generator::new_b(starting_numbers.1);

    a
        .zip(b)
        .map(|(a, b)| have_matching_lower_16_bits(a, b))
        .take(40000000)
        .map(|b| if b { 1 } else { 0 })
        .sum()
}

fn main() {
    let path = "input.txt";
    let starting_numbers = get_starting_numbers(path);
    println!("Day 15, part 1: {:?}", get_part_1(starting_numbers));
}
