use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Program {
    id: u32,
    connected: Vec<u32>,
}

fn parse_program(line: &str) -> Program {
    let pieces: Vec<&str> = line
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|&p| p.len() > 0)
        .collect();

    let id = pieces[0].parse::<u32>().expect("Could not parse a program ID.");
    let connected = pieces
        .iter()
        .skip(2)
        .map(|p| p.parse::<u32>().expect("Could not parse a connected program ID."))
        .collect();

    Program {
        id,
        connected,
    }
}

fn parse_programs(path: &str) -> HashMap<u32, Program> {
    let f = File::open(path).expect("Could not open the specified file.");
    let reader = BufReader::new(f);
    let mut programs = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Could not read a line.");
        let program = parse_program(&line);
        programs.insert(program.id, program);
    }

    programs
}

fn get_group(programs: &HashMap<u32, Program>, start_id: u32) -> HashSet<u32> {
    let mut visited = HashSet::new();
    let mut pending = vec![start_id];

    while pending.len() > 0 {
        let current_id = pending.pop().unwrap();
        let current_program = programs.get(&current_id).unwrap();
        visited.insert(current_id);

        for connected_id in &current_program.connected {
            if visited.contains(&connected_id) {
                continue;
            }

            pending.push(*connected_id);
        }
    }
    
    visited
}

fn count_groups(programs: &HashMap<u32, Program>) -> usize {
    let mut remaining: HashSet<u32> = programs.keys().map(|k| *k).collect();
    let mut group_count = 0;

    while remaining.len() > 0 {
        let next_id = *remaining.iter().nth(0).unwrap();
        let group_ids = get_group(programs, next_id);
        for group_id in group_ids {
            remaining.remove(&group_id);
        }

        group_count += 1;
    }

    group_count
}

fn main() {
    let path = "input.txt";
    println!("Day 11, part 1: {}", get_group(&parse_programs(path), 0).len());
    println!("Day 11, part 2: {}", count_groups(&parse_programs(path)));
}
