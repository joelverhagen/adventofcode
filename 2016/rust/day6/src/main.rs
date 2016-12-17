use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum ColumnFileError {
    CouldNotReadFile,
}

#[derive(Debug, Copy, Clone)]
enum CharSelection {
    LeastCommon,
    MostCommon,
}

fn get_most_frequent_letter(chars: &Vec<char>, selection: CharSelection) -> Option<char> {
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
        match a.1.cmp(b.1) { 
            Ordering::Equal => a.0.cmp(b.0),
            other           => other,
        }
    });

    let entry = match selection {
        CharSelection::LeastCommon => char_counts.first(),
        CharSelection::MostCommon  => char_counts.last(),
    };

    Some(*entry.unwrap().0)
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

fn select_column_letters(columns: &Vec<Vec<char>>, selection: CharSelection) -> String {
    let mut output = String::new();

    for column in columns {
        match get_most_frequent_letter(&column, selection) {
            Some(c) => output.push(c),
            None    => {},
        };
    }

    output
}

fn main() {
    let columns = get_char_columns("input.txt").unwrap();

    let part_1_result = select_column_letters(&columns, CharSelection::MostCommon);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = select_column_letters(&columns, CharSelection::LeastCommon);
    println!("Part 2 result: {}", part_2_result);
}
