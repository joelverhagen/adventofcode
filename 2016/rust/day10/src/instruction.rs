use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
pub enum InstructionError {
    CouldNotOpenFile(io::Error),
    CouldNotReadFile(io::Error),
    CouldNotParseInteger,
    CouldNotParseDestination,
    CouldNotParseInstruction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Bot(usize);

impl Bot {
    pub fn new(value: usize) -> Bot {
        Bot(value)
    }

    pub fn value(&self) -> usize {
        match self {
            &Bot(value) => value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Microchip(usize);

impl Microchip {
    pub fn new(value: usize) -> Microchip {
        Microchip(value)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Output(usize);

impl Output {
    pub fn new(value: usize) -> Output {
        Output(value)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Destination {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
pub enum Instruction {
    MicrochipGoesTo(Microchip, Bot),
    BotGives(Bot, Destination, Destination),
}

impl Instruction {
    pub fn parse_file(path: &str) -> Result<Vec<Instruction>, InstructionError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(InstructionError::CouldNotOpenFile(err)),
        };

        let mut output: Vec<Instruction> = Vec::new();

        let file_reader = BufReader::new(file);
        for line_result in file_reader.lines() {
            let line = match line_result {
                Ok(line) => line,
                Err(err) => return Err(InstructionError::CouldNotReadFile(err)),
            };

            let instruction = Instruction::parse(&line)?;
            output.push(instruction);
        }

        Ok(output)
    }

    pub fn parse(unparsed_instruction: &str) -> Result<Instruction, InstructionError> {
        lazy_static! {
            static ref MICROCHIP_GOES_TO: Regex = Regex::new("^\
                value (?P<microchip>\\d+) \
                goes to \
                bot (?P<bot>\\d+)").unwrap();
            static ref BOT_GIVES: Regex = Regex::new("^\
                bot (?P<bot>\\d+) gives low to \
                (?P<low_type>bot|output) (?P<low_id>\\d+) \
                and high to \
                (?P<high_type>bot|output) (?P<high_id>\\d+)$").unwrap();
        }

        match MICROCHIP_GOES_TO.captures(unparsed_instruction) {
            Some(caps) => {
                let microchip = parse_integer(caps.get(1).unwrap().as_str())?;
                let bot = parse_integer(caps.get(2).unwrap().as_str())?;

                return Ok(Instruction::MicrochipGoesTo(Microchip(microchip), Bot::new(bot)));
            },
            None       => {}
        };

        match BOT_GIVES.captures(unparsed_instruction) {
            Some(caps) => {
                let bot = parse_integer(caps.get(1).unwrap().as_str())?;
                let low_id = parse_integer(caps.get(3).unwrap().as_str())?;
                let low = parse_type(caps.get(2).unwrap().as_str(), low_id)?;
                let high_id = parse_integer(caps.get(5).unwrap().as_str())?;
                let high = parse_type(caps.get(4).unwrap().as_str(), high_id)?;

                return Ok(Instruction::BotGives(Bot::new(bot), low, high));
            },
            None       => {}
        };

        Err(InstructionError::CouldNotParseInstruction)
    }
}

fn parse_integer(input: &str) -> Result<usize, InstructionError> {
    match input.parse::<usize>() {
        Ok(i)  => Ok(i),
        Err(_) => Err(InstructionError::CouldNotParseInteger),
    }
}

fn parse_type(input: &str, id: usize) -> Result<Destination, InstructionError> {
    match input {
        "bot"    => Ok(Destination::Bot(id)),
        "output" => Ok(Destination::Output(id)),
        _        => Err(InstructionError::CouldNotParseDestination),
    }
}