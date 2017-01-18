#[macro_use] extern crate lazy_static;
extern crate regex;

mod factorystate;
mod instruction;
mod instructionprocessor;

use instruction::Microchip;
use instruction::Instruction;
use instructionprocessor::InstructionProcessor;

fn main() {
    let path = "input.txt";
    let instructions = Instruction::parse_file(path).unwrap();
    let comparison = InstructionProcessor::process_and_find_comparison(
        instructions,
        Microchip::new(17),
        Microchip::new(61)).unwrap().unwrap().value();

    println!("Part 1 result: {}", comparison);
} 
