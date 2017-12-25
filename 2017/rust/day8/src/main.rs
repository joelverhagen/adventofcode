use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
enum Operator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: Operator,
    value: i32,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    increase: bool,
    value: i32,
    condition: Condition,
}

fn parse_instruction(line: &str) -> Instruction {
    let pieces: Vec<&str> = line.split_whitespace().collect();
    
    let register = String::from(pieces[0]);
    let increase = match pieces[1] {
        "inc" => true,
        "dec" => false,
        _     => panic!("Expected 'inc' or 'dec'."),
    };
    let value = pieces[2].parse::<i32>().expect("Could not parse instruction value as i32.");

    let condition_register = String::from(pieces[4]);
    let condition_operator = match pieces[5] {
        "<"  => Operator::LessThan,
        "<=" => Operator::LessThanOrEqual,
        ">"  => Operator::GreaterThan,
        ">=" => Operator::GreaterThanOrEqual,
        "==" => Operator::Equal,
        "!=" => Operator::NotEqual,
        _    => panic!("Unexpected condition operator."),
    };
    let condition_value = pieces[6].parse::<i32>().expect("Could not parse condition value as i32.");

    Instruction {
        register,
        increase,
        value,
        condition: Condition {
            register: condition_register,
            operator: condition_operator,
            value: condition_value,
        },
    }
}

fn parse_file(file_name: &str) -> Vec<Instruction> {
    let f = File::open(file_name).expect("Could not open the specified file.");
    let reader = BufReader::new(f);
    
    reader
        .lines()
        .map(|lr| lr.expect("Could not read a line."))
        .map(|l| parse_instruction(&l))
        .collect()
}

fn process_instructions(instructions: &Vec<Instruction>) -> (HashMap<&str, i32>, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut max = 0;

    for instruction in instructions {
        let current = *registers.entry(&instruction.condition.register).or_insert(0);

        let condition_satisfied = match instruction.condition.operator {
            Operator::LessThan           => current <  instruction.condition.value,
            Operator::LessThanOrEqual    => current <= instruction.condition.value,
            Operator::GreaterThan        => current >  instruction.condition.value,
            Operator::GreaterThanOrEqual => current >= instruction.condition.value,
            Operator::Equal              => current == instruction.condition.value,
            Operator::NotEqual           => current != instruction.condition.value,
        };

        if !condition_satisfied {
            continue;
        }

        let delta = match instruction.increase {
            true  => instruction.value,
            false => -1 * instruction.value,
        };

        let entry = registers.entry(&instruction.register).or_insert(0);
        *entry += delta;
        let new_value = *entry;

        if new_value > max {
            max = new_value;
        }
    }

    (registers, max)
}

fn get_largest_register_value(registers: &HashMap<&str, i32>) -> i32 {
    *registers
        .iter()
        .map(|(_, v)| v)
        .max()
        .unwrap_or(&0)
}

fn main() {
    let file_name = "input.txt";
    let instructions = parse_file(file_name);
    let (registers, largest_value) = process_instructions(&instructions);
    println!("Day 8, part 1: {}", get_largest_register_value(&registers));
    println!("Day 8, part 2: {}", largest_value);
}
