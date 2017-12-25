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

fn find_imbalance(programs: &HashMap<String, Program>) -> i32 {
    let mut total_weights: HashMap<&String, u32> = HashMap::new();
    
    while total_weights.len() < programs.len() {
        for program in programs.values() {
            let mut above_weights: HashMap<&String, u32> = HashMap::new();
            for above in &program.above {
                let entry = total_weights.get(&above);
                if entry.is_none() {
                    break;
                } else {
                    above_weights.insert(above, *entry.unwrap());
                }
            }

            if above_weights.len() < program.above.len() {
                continue;
            }

            let actual_total_above = above_weights
                .values()
                .map(|x| *x)
                .sum::<u32>();

            // We can select an expected 
            if above_weights.len() > 2 {
                let mut weight_to_count: HashMap<u32, usize> = HashMap::new();
                for (_, value) in &above_weights {
                    *weight_to_count.entry(*value).or_insert(0) += 1;
                }

                let mut sorted_weight_to_count: Vec<(&u32, &usize)> = weight_to_count
                    .iter()
                    .collect();
                sorted_weight_to_count.sort_by(|a, b| b.1.cmp(a.1));

                let correct_above_weight = *sorted_weight_to_count[0].0;
                let expected_total_above = correct_above_weight * above_weights.len() as u32;
                let extra_weight = expected_total_above as i32 - actual_total_above as i32;

                if extra_weight != 0 {
                    for above in &program.above {
                        if *above_weights.get(above).unwrap() != correct_above_weight {
                            return programs.get(above).unwrap().weight as i32 + extra_weight;
                        }
                    }
                }
            }

            let total_weight = program.weight + actual_total_above;

            total_weights.insert(&program.name, total_weight);
        }
    }

    0
}

fn main() {
    let file_name = "input.txt";
    let programs = read_programs(file_name);
    println!("Day 7, part 1: {}", find_root(&programs));
    println!("Day 7, part 2: {}", find_imbalance(&programs));
}
