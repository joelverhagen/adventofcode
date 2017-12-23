use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_spreadsheet(file_name: &str) -> Vec<Vec<i32>> {
    let fh = File::open(file_name).expect("Failed to open the specified file.");
    let reader = BufReader::new(fh);
    let mut spreadsheet: Vec<Vec<i32>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|cell| cell.parse::<i32>().unwrap()).collect();
        spreadsheet.push(row);
    }
    
    spreadsheet
}

fn get_max_difference(row: &Vec<i32>) -> i32 {
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for cell in row {
        if *cell < min {
            min = *cell;
        }

        if *cell > max {
            max = *cell;
        }
    }

    max - min
}

fn get_quotient(row: &Vec<i32>) -> i32 {
    for i in 0..row.len() {
        for j in 0..i {
            let mut dividend = row[i];
            let mut divisor = row[j];

            if dividend < divisor {
                let temp = dividend;
                dividend = divisor;
                divisor = temp;
            }
            
            if dividend % divisor == 0 {
                return dividend / divisor;
            }
        }
    }

    panic!("No even divisor found.");
}

fn calculate_checksum(spreadsheet: &Vec<Vec<i32>>, f: &Fn(&Vec<i32>) -> i32) -> i32 {
    let mut checksum = 0;
    for row in spreadsheet {
        checksum += f(row);
    }

    checksum
}


fn main() {
    let file_name = "input.txt";
    let spreadsheet = read_spreadsheet(&file_name);
    println!("Day 2, part 1: {}", calculate_checksum(&spreadsheet, &get_max_difference));
    println!("Day 2, part 2: {}", calculate_checksum(&spreadsheet, &get_quotient));
}
