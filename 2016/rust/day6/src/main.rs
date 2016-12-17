use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum ColumnFileError {
    CouldNotReadFile,
}

fn get_most_frequent_letter(chars: &Vec<char>) -> Option<char> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();

    for c in chars {
        let entry = char_counts.entry(*c).or_insert(0);
        *entry += 1;
    }

    if char_counts.len() == 0 {
        return None;
    }

    let mut char_counts: Vec<(&char, &i32)> = char_counts
        .iter()
        .collect();

    char_counts.sort_by(|a, b| {
        match a.1.cmp(b.1).reverse() { 
            Ordering::Equal => a.0.cmp(b.0),
            other           => other,
        }
    });

    let c = *char_counts
        .iter()
        .nth(0)
        .unwrap()
        .0;

    Some(c)
}

fn get_char_columns(path: &str) -> Result<Vec<Vec<char>>, ColumnFileError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_)   => return Err(ColumnFileError::CouldNotReadFile),
    };

    let file_reader = BufReader::new(file);
    let mut columns: Vec<Vec<char>> = Vec::new();

    for line_result in file_reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_)   => return Err(ColumnFileError::CouldNotReadFile),
        };

        let mut column_index = 0;

        for c in line.chars() {
            if column_index >= columns.len() {
                columns.push(Vec::new());
            }

            columns[column_index].push(c);
            column_index += 1;
        }
    }

    Ok(columns)
}

fn get_part_1_result(columns: &Vec<Vec<char>>) -> String {
    let mut output = String::new();

    for column in columns {
        match get_most_frequent_letter(&column) {
            Some(c) => output.push(c),
            None    => {},
        };
    }

    output
}

fn main() {
    let columns = get_char_columns("input.txt").unwrap();

    let part_1_result = get_part_1_result(&columns);
    println!("Part 1 result: {}", part_1_result);
}
