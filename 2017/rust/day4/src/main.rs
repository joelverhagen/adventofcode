use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn parse_password_list(file_name: &str) -> Vec<Vec<String>> {
    let f = File::open(file_name).expect("Failed to open the specified file.");
    let reader = BufReader::new(f);
    let mut output: Vec<Vec<String>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Could not read a line.");
        let words: Vec<String> = line.split_whitespace().map(|s| String::from(s)).collect();
        output.push(words);
    }

    output
}

fn has_no_unique_words(passphrase: &Vec<String>) -> bool {
    let mut unique: HashSet<&str> = HashSet::new();

    for word in passphrase {
        if !unique.insert(word) {
            return false;
        }
    }

    true
}

fn has_no_anagrams(passphrase: &Vec<String>) -> bool {
    let mut unique: HashSet<String> = HashSet::new();

    for word in passphrase {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_word: String = chars.into_iter().collect();
        if !unique.insert(sorted_word) {
            return false;
        }
    }

    true
}

fn count_valid_passphrases(file_name: &str, f: &Fn(&Vec<String>) -> bool) -> usize {
    parse_password_list(file_name)
        .iter()
        .filter(|x| f(x))
        .count()
}

fn main() {
    let file_name = "input.txt";
    println!("Day 4, part 1: {}", count_valid_passphrases(file_name, &has_no_unique_words));
    println!("Day 4, part 2: {}", count_valid_passphrases(file_name, &has_no_anagrams));
}
