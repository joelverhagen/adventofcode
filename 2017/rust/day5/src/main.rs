use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn read_file(file_name: &str) -> Vec<i32> {
    let f = File::open(file_name).expect("Filed to open the specified file.");
    let reader = BufReader::new(f);
    let mut output: Vec<i32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Could not read a line.");
        let jump = line.parse::<i32>().expect("Could not parse line as i32.");
        output.push(jump);
    }

    output
}

fn process_jumps(jumps: &mut Vec<i32>, f: &Fn(i32) -> i32) -> usize {
    let mut index: i32 = 0;
    let mut count = 0;

    while index >= 0 && index < jumps.len() as i32 {
        let jump = jumps[index as usize];
        jumps[index as usize] += f(jump);
        count += 1;

        index += jump;
    }
    
    count
}

fn main() {
    let file_name = "input.txt";
    println!("Day 5, part 1: {}", process_jumps(&mut read_file(file_name), &|_| 1));
    println!("Day 5, part 2: {}", process_jumps(&mut read_file(file_name), &|j| if j >= 3 { -1 } else { 1 }));
}
