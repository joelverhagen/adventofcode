#[macro_use] extern crate lazy_static;
extern crate regex;

mod factorystate;
mod instruction;
mod instructionprocessor;

use instruction::Microchip;
use instruction::Instruction;
use instruction::Output;
use instructionprocessor::InstructionProcessor;

fn main() {
    let path = "input.txt";

    let instructions = Instruction::parse_file(path).unwrap();
    let comparison = InstructionProcessor::process_and_find_comparison(
        instructions,
        Microchip::new(17),
        Microchip::new(61)).unwrap().unwrap().value();
    println!("Part 1 result: {}", comparison);

    let instructions = Instruction::parse_file(path).unwrap();
    let product = InstructionProcessor::process_and_find_output_product(
        instructions,
        &vec![Output::new(0), Output::new(1), Output::new(2)]).unwrap();
    println!("Part 2 result: {}", product);
} 
