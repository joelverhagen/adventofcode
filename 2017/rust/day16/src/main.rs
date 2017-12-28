use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::prelude::*;

fn rotate<T>(slice: &mut [T], n: usize) {
    let length = slice.len();
    slice.reverse();
    slice[0..n].reverse();
    slice[n..length].reverse();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Programs {
    order: Vec<char>,
    indices: HashMap<char, usize>,
}

impl Hash for Programs {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.order.hash(state);
    }
}

impl Programs {
    fn new(count: usize) -> Programs {
        let order: Vec<char> = (0..count)
            .map(|c| (((('a' as u8) as usize) + c) as u8) as char)
            .collect();
        
        let mut output = Programs {
            order,
            indices: HashMap::new(),
        };

        output.update_indices();

        output
    }

    fn get_order(&self) -> String {
        self.order.iter().collect()
    }

    fn execute_dance_moves<'a, I>(&mut self, dance_moves: I) where I: Iterator<Item = &'a DanceMove> {
        for dance_move in dance_moves {
            self.execute_dance_move(dance_move);
        }
    }
    
    fn execute_dance_move(&mut self, dance_move: &DanceMove) {
        match dance_move {
            &DanceMove::Spin(count)        => {
                rotate(self.order.as_mut_slice(), count);
                self.update_indices();
            },
            &DanceMove::Exchange(a_i, b_i) => {
                self.order.swap(a_i, b_i);
                self.update_indices();
            },
            &DanceMove::Partner(a, b)      => {
                let a_i = self.indices[&a];
                let b_i = self.indices[&b];
                self.execute_dance_move(&DanceMove::Exchange(a_i, b_i));
            }
        }
    }

    fn update_indices(&mut self) {
        for i in 0..self.order.len() {
            let p = self.order[i].clone();
            *self.indices.entry(p).or_insert(i) = i;
        }
    }
}


fn parse_dance_move(input: &str) -> DanceMove {
    let trimmed = input.trim();

    match trimmed.chars().nth(0).unwrap() {
        's' => {
            let count = trimmed[1..].parse::<usize>().expect("Could not parse a spin dance move.");
            DanceMove::Spin(count)
        },
        'x' => {
            let pieces: Vec<&str> = trimmed[1..].split('/').collect();
            let a = pieces[0].parse::<usize>().expect("Could not parse the first exchange position.");
            let b = pieces[1].parse::<usize>().expect("Could not parse the second exchange position.");
            DanceMove::Exchange(a, b)
        },
        'p' => {
            let pieces: Vec<&str> = trimmed[1..].split('/').collect();
            let a = pieces[0].chars().nth(0).unwrap();
            let b = pieces[1].chars().nth(0).unwrap();
            DanceMove::Partner(a, b)
        },
        _   => panic!("Unexpected dance move."),
    }    
}

fn parse_dance_moves(path: &str) -> Vec<DanceMove> {
    let mut f = File::open(path).expect("The specified file path could not be opened.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("The file could not be read to a string.");

    let pieces: Vec<&str> = contents.split(',').collect();
    let mut moves = Vec::new();

    for piece in pieces {
        moves.push(parse_dance_move(piece));
    }

    moves
}

fn get_part_1(path: &str, count: usize) -> String {
    let mut programs = Programs::new(count);
    let dance_moves = parse_dance_moves(path);
    
    programs.execute_dance_moves(dance_moves.iter());

    programs.get_order()
}

fn get_part_2(path: &str, count: usize) -> String {
    let mut programs = Programs::new(count);
    let dance_moves = parse_dance_moves(path);
    let mut programs_to_i = HashMap::new();
    let mut states = Vec::new();
    
    let mut loop_size = <usize>::max_value();
    for i in 0..1000000000 {
        if programs_to_i.insert(programs.clone(), i).is_some() {
            loop_size = i;
            break;
        }

        states.push(programs.clone());

        programs.execute_dance_moves(dance_moves.iter());
    }

    states[1000000000 % loop_size].get_order()
}

fn main() {
    let count = 16;
    let path = "input.txt"; 
    println!("Day 16, part 1: {}", get_part_1(path, count));
    println!("Day 16, part 2: {}", get_part_2(path, count));
}
