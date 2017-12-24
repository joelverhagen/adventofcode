use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct Program {
    line_index: usize,
    name: String,
    weight: u32,
    above: Vec<String>,
}

fn parse_line(line_index: usize, line: &str) -> Program {
    let pieces: Vec<&str> = line.split_whitespace().collect();
    
    let name = String::from(pieces[0]);

    let weight = pieces[1][1..pieces[1].len() - 1]
        .parse::<u32>()
        .expect("Could not parse the weight.");

    let mut above = Vec::new();
    for i in 3..pieces.len() {
        let piece = pieces[i];
        let mut end = piece.len();
        if i < pieces.len() - 1 {
            end -= 1;
        }

        let above_name = String::from(&piece[0..end]);
        above.push(above_name);
    }   

    Program {
        line_index,
        name,
        weight,
        above,
    }
}

fn read_programs(file_name: &str) -> HashMap<String, Program> {
    let f = File::open(file_name).expect("Could not open the specified file.");
    let reader = BufReader::new(f);
    
    let mut programs = HashMap::new();
    let mut line_number = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("Could not read a line.");
        let program = parse_line(line_number, &line);
        programs.insert(program.name.clone(), program);
        line_number += 1;
    }

    programs
}

fn find_root(programs: &HashMap<String, Program>) -> &String {
    let mut candidates: HashSet<&String> = HashSet::from_iter(programs.keys());

    for program in programs.values() {
        for above in &program.above {
            candidates.remove(&above);
        }
    }
    
    let root = candidates.drain().nth(0).unwrap();
    root
}

fn main() {
    let file_name = "input.txt";
    let programs = read_programs(file_name);
    println!("Day 7, part 1: {}", find_root(&programs));
}
