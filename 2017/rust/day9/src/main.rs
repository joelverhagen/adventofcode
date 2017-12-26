use std::fs::File;
use std::str::Chars;
use std::io::prelude::*;

#[derive(Debug)]
enum Token {
    StartGroup,
    EndGroup,
    Separator,
    StartGarbage,
    EndGarbage,
    Escape,
    Escaped(char),
    Other(char),
}

struct Tokens<'a> {
    chars: Chars<'a>,
    escaped: bool,
    in_garbage: bool,
}

impl<'a> Tokens<'a> {
    fn new(input: &str) -> Tokens {
        Tokens {
            chars: input.chars(),
            escaped: false,
            in_garbage: false,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(c) => Some(match c {
                '{' if !self.escaped && !self.in_garbage => Token::StartGroup,
                '}' if !self.escaped && !self.in_garbage => Token::EndGroup,
                ',' if !self.escaped && !self.in_garbage => Token::Separator,
                '<' if !self.escaped && !self.in_garbage => { self.in_garbage = true; Token::StartGarbage },
                '>' if !self.escaped                     => { self.in_garbage = false; Token::EndGarbage },
                '!' if !self.escaped                     => { self.escaped = true; Token::Escape },
                _   if !self.escaped                     => { self.escaped = false; Token::Other(c) },
                _                                        => { self.escaped = false; Token::Escaped(c) },
            }),
            None    => None,
        }
    }
}

fn calculate_score<'a>(tokens: Tokens<'a>) -> u32 {
    let mut score = 0;
    let mut depth = 0;

    for token in tokens {
        match token {
            Token::StartGroup => { depth += 1; score += depth; },
            Token::EndGroup   => { depth -= 1; },
            _                 => {},
        }
    }

    score
}

fn count_garbage<'a>(tokens: Tokens<'a>) -> u32 {
    let mut count = 0;
    let mut in_garbage = false;
    
    for token in tokens {
        match token {
            Token::StartGarbage           => in_garbage = true,
            Token::EndGarbage             => in_garbage = false,
            Token::Other(c) if in_garbage => count += 1,
            _                             => {},
        };
    }

    count
}

fn read_contents(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("Could not open the specified file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read the file.");
    
    contents
}

fn main() {
    let file_name = "input.txt";
    let contents = read_contents(file_name);
    println!("Day 9, part 1: {}", calculate_score(Tokens::new(&contents)));
    println!("Day 9, part 2: {}", count_garbage(Tokens::new(&contents)));
}
