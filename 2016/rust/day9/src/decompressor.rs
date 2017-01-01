use std::io;
use std::iter::Peekable;
use decompresserror::DecompressError;
use decompresstokens::DecompressTokens;
use decompresstokens::DecompressToken;

pub struct Decompressor {
    tokens: Peekable<DecompressTokens>,
    state: State,
    repeat_sequence: Vec<char>,
    repeat_length_remaining: usize,
    repeat_count_remaining: usize,
}

#[derive(Debug)]
enum State {
    Initial,
    RepeatDirective,
    Error,
}

impl Decompressor  {
    pub fn open(path: &str) -> Result<Decompressor, io::Error> {
        let tokens = match DecompressTokens::open(path) {
            Ok(tokens) => tokens,
            Err(err)   => return Err(err),
        };

        Ok(Decompressor {
            tokens: tokens.peekable(),
            state: State::Initial,
            repeat_sequence: Vec::new(),
            repeat_length_remaining: 0,
            repeat_count_remaining: 0,
        })
    }

    pub fn read_to_end(&mut self) -> Result<String, DecompressError> {
        let mut output = String::new();
        
        for c_result in self {
            match c_result {
                Ok(c)    => output.push(c),
                Err(err) => return Err(err),
            }
        }   

        Ok(output)
    }

    fn start_repeat_directive(&mut self) -> Option<Result<char, DecompressError>> {
        match self.consume_repeat_directive() {
            Ok(())   => {},
            Err(err) => return self.error(err),
        };

        match self.consume_repeat_sequence() {
            Ok(())   => {},
            Err(err) => return self.error(err),
        };

        self.state = State::RepeatDirective;
        self.next_repeat_character()
    }

    fn consume_repeat_directive(&mut self) -> Result<(), DecompressError> {
        match (self.tokens.next(),
               self.tokens.next(),
               self.tokens.next(),
               self.tokens.next(),
               self.tokens.next()) {
              (Some(Ok(DecompressToken::OpenParenthesis)),
               Some(Ok(DecompressToken::Integer(length))),
               Some(Ok(DecompressToken::X)),
               Some(Ok(DecompressToken::Integer(count))),
               Some(Ok(DecompressToken::CloseParenthesis))) => {
                self.repeat_length_remaining = length;
                self.repeat_count_remaining = count;
                Ok(())
              },
              _                                   => {
                Err(DecompressError::InvalidRepeatDirective)
              },
        }
    }

    fn consume_repeat_sequence(&mut self) -> Result<(), DecompressError> {
        self.repeat_sequence.clear();

        for _ in 0..self.repeat_length_remaining {
            match self.tokens.next() {
                Some(Ok(DecompressToken::Character(c))) => self.repeat_sequence.push(c),
                _                                       => return Err(DecompressError::ExpectedMoreCharacters),
            };
        }
        
        Ok(())
    }

    fn next_repeat_character(&mut self) -> Option<Result<char, DecompressError>> {
        let c = self.repeat_sequence[self.repeat_sequence.len() - self.repeat_length_remaining];

        self.repeat_length_remaining -= 1;
        
        if self.repeat_length_remaining == 0 {
            self.repeat_count_remaining -= 1;
            if self.repeat_count_remaining == 0 {
                self.state = State::Initial;
            } else {
                self.repeat_length_remaining = self.repeat_sequence.len();
            }
        }

        Some(Ok(c))
    }

    fn character(&mut self, c: char) -> Option<Result<char, DecompressError>> {
        self.tokens.next();
        Some(Ok(c))
    }

    fn error(&mut self, err: DecompressError) -> Option<Result<char, DecompressError>> {
        self.state = State::Error;
        Some(Err(err))
    }
}

impl Iterator for Decompressor {
    type Item = Result<char, DecompressError>;

    fn next(&mut self) -> Option<Result<char, DecompressError>> {
        match self.state {
            State::Initial         => {                        
                let t = match self.tokens.peek() {
                    Some(&Ok(t))    => t,
                    Some(&Err(err)) => return Some(Err(err)),
                    None            => return None,
                };

                match t {
                    DecompressToken::Character(c)    => self.character(c),
                    DecompressToken::OpenParenthesis => self.start_repeat_directive(),
                    _                                => self.error(DecompressError::UnexpectedToken(t))
                }
            },
            State::RepeatDirective => self.next_repeat_character(),
            State::Error           => None,
        }
    }
}
