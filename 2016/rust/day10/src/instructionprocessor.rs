use std::collections::VecDeque;
use instruction::Bot;
use instruction::Instruction;
use instruction::Microchip;
use factorystate::BotState;
use factorystate::FactoryState;

#[derive(Debug)]
pub enum InstructionProcessorError {
    CouldNotProcessInstructions
}

#[derive(Debug)]
pub struct BotComparison {
    bot: Bot,
    low: Microchip,
    high: Microchip,
}

pub struct InstructionProcessor;

impl InstructionProcessor {
    fn find_comparison(comparisons: &Vec<BotComparison>, low: Microchip, high: Microchip) -> Option<Bot> {
        for comparison in comparisons {
            if comparison.low == low && comparison.high == high {
                return Some(comparison.bot);
            }
        }

        None
    }

    pub fn process_and_find_comparison(instructions: Vec<Instruction>, low: Microchip, high: Microchip) -> Result<Option<Bot>, InstructionProcessorError> {
        let (_, comparisons) = InstructionProcessor::process(instructions)?;
        let comparison_option = InstructionProcessor::find_comparison(&comparisons, low, high);

        Ok(comparison_option)
    }

    pub fn process(instructions: Vec<Instruction>) -> Result<(FactoryState, Vec<BotComparison>), InstructionProcessorError> {
        let mut state = FactoryState::new();
        let mut instruction_queue: VecDeque<Instruction> = VecDeque::new();
        let mut comparisons: Vec<BotComparison> = Vec::new();

        for instruction in instructions {
            instruction_queue.push_back(instruction);
        }

        let mut err_count = 0;

        while instruction_queue.len() > 0 && err_count < instruction_queue.len() {
            let instruction = instruction_queue.pop_front().unwrap();

            let bot_comparison = match &instruction {
                &Instruction::BotGives(bot, _, _) => {
                    match state.get_bot_state(bot) {
                        BotState { low: Some(low), high: Some(high) } => Some(BotComparison {
                            bot: bot,
                            low: low,
                            high: high,
                        }),
                        _                                             => None,
                    }
                },
                _                    => None,
            };

            match bot_comparison {
                Some(bot_comparison) => comparisons.push(bot_comparison),
                None                 => {},
            };

            match state.execute_instruction(&instruction) {
                Ok(()) => {
                    err_count = 0;
                },
                Err(_) => {
                    err_count += 1;
                    instruction_queue.push_back(instruction);
                },
            }
        }

        match err_count {
            0 => Ok((state, comparisons)),
            _ => Err(InstructionProcessorError::CouldNotProcessInstructions),
        }
    }
}
