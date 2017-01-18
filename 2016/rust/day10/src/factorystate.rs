use std::collections::HashMap;
use instruction::Bot;
use instruction::Destination;
use instruction::Instruction;
use instruction::Microchip;
use instruction::Output;

#[derive(Debug)]
pub enum FactoryStateError {
    BotAlreadyHasTwoMicrochips,
    BotDoesNotHaveTwoMicrochips,
    OutputAlreadyHasMicrochip,
}

#[derive(Debug)]
pub struct FactoryState {
    bots: HashMap<Bot, BotState>,
    output: HashMap<Output, Option<Microchip>>,
}

#[derive(Clone, Copy, Debug)]
pub struct BotState {
    pub low: Option<Microchip>,
    pub high: Option<Microchip>,
}

impl FactoryState {
    pub fn new() -> FactoryState {
        FactoryState {
            bots: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn get_bot_state(&mut self, bot: Bot) -> BotState {
        *self.bots.entry(bot).or_insert(BotState {
            low: None,
            high: None,
        })
    }

    pub fn get_output_microchip(&self, output: &Output) -> Option<Microchip> {
        match self.output.get(output) {
            Some(microchip) => *microchip,
            None            => None
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), FactoryStateError> {
        match instruction {
            &Instruction::MicrochipGoesTo(microchip, bot) => self.execute_microchip_goes_to_bot(microchip, bot),
            &Instruction::BotGives(bot, low, high)        => self.execute_bot_gives(bot, low, high),
        }
    }

    fn execute_microchip_goes_to_bot(&mut self, microchip: Microchip, bot: Bot) -> Result<(), FactoryStateError> {
        let bot = self.bots.entry(bot).or_insert(BotState {
            low: None,
            high: None,
        });

        *bot = match *bot {
            BotState { low: None,      high: None       }                     => BotState { low: Some(microchip), high: None },
            BotState { low: Some(low), high: None       } if microchip > low  => BotState { low: Some(low), high: Some(microchip) },
            BotState { low: Some(low), high: None       }                     => BotState { low: Some(microchip), high: Some(low) },
            BotState { low: None,      high: Some(high) } if microchip < high => BotState { low: Some(microchip), high: Some(high) },
            BotState { low: None,      high: Some(high) }                     => BotState { low: Some(high), high: Some(microchip) },
            BotState { low: Some(_),   high: Some(_)    }                     => return Err(FactoryStateError::BotAlreadyHasTwoMicrochips),
        };

        Ok(())
    }

    fn execute_microchip_goes_to_output(&mut self, microchip: Microchip, output: Output) -> Result<(), FactoryStateError> {
        let output = self.output.entry(output).or_insert(None);

        *output = match *output {
            None    => Some(microchip),
            Some(_) => return Err(FactoryStateError::OutputAlreadyHasMicrochip),
        };

        Ok(())
    }

    fn execute_microchip_goes_to_destination(&mut self, microchip: Microchip, destination: Destination) -> Result<(), FactoryStateError> {
        match destination {
            Destination::Bot(value)    => self.execute_microchip_goes_to_bot(microchip, Bot::new(value)),
            Destination::Output(value) => self.execute_microchip_goes_to_output(microchip, Output::new(value)),
        }
    }

    fn execute_bot_gives(&mut self, bot: Bot, low: Destination, high: Destination) -> Result<(), FactoryStateError> {
        let low_microchip;
        let high_microchip;

        {
            let bot_entry = self.bots.entry(bot).or_insert(BotState {
                low: None,
                high: None,
            });

            let (low, high) = match *bot_entry {
                BotState { low: Some(low), high: Some(high) } => (low, high),
                _                                             => return Err(FactoryStateError::BotDoesNotHaveTwoMicrochips),
            };

            low_microchip = low;
            high_microchip = high;
        }

        self.execute_microchip_goes_to_destination(low_microchip, low)?;
        self.execute_microchip_goes_to_destination(high_microchip, high)?;

        {
            let bot_entry = self.bots.entry(bot).or_insert(BotState {
                low: None,
                high: None,
            });

            (*bot_entry).low = None;
            (*bot_entry).high = None;
        }

        Ok(())
    }
}