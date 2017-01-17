use std::io;
use std::iter::Peekable;
use decompresserror::DecompressError;
use filechars::FileChars;

#[derive(Clone, Copy, Debug)]
pub enum DecompressTokenType {
    OpenParenthesis,
    Integer(usize),
    X,
    CloseParenthesis,
    Character(char),
}

#[derive(Debug)]
pub struct DecompressToken {
    pub text: Vec<char>,
    pub token_type: DecompressTokenType,
}

impl DecompressToken {
    fn new(text: Vec<char>, token_type: DecompressTokenType) -> DecompressToken {
        DecompressToken {
            text: text,
            token_type: token_type,
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Initial,
    ExpectingFirstInteger,
    ExpectingSecondInteger,
    ExpectingX,
    ExpectingCloseParenthesis,
    Error,
}

pub struct DecompressTokens {
    chars: Peekable<FileChars>,
    state: State,
}

impl DecompressTokens {
    pub fn open(path: &str) -> Result<DecompressTokens, io::Error> {
        let chars = match FileChars::open(path) {
            Ok(chars) => chars,
            Err(err)  => return Err(err),
        };

        Ok(DecompressTokens {
            chars: chars.peekable(),
            state: State::Initial,
        })
    }

    fn read_integer(&mut self, next_state: State) -> Result<DecompressToken, DecompressError> {        
        match read_integer(&mut self.chars) {
            Ok((text, i))    => {
                self.state = next_state;
                Ok(DecompressToken::new(text, DecompressTokenType::Integer(i)))
            },
            Err(err) => {
                self.state = State::Error;
                Err(err)
            }
        }
    }

    fn token(&mut self, c: char, next_state: State, token_type: DecompressTokenType) -> Result<DecompressToken, DecompressError> {
        let text = vec![c];
        self.chars.next();
        self.state = next_state;
        Ok(DecompressToken::new(text, token_type))
    }

    fn error(&mut self, err: DecompressError) -> Result<DecompressToken, DecompressError> {   
        self.chars.next();
        self.state = State::Error;
        Err(err)
    }
}

impl Iterator for DecompressTokens {
    type Item = Result<DecompressToken, DecompressError>;

    fn next(&mut self) -> Option<Result<DecompressToken, DecompressError>> {
        let c = match self.chars.peek() {
            Some(&Ok(c))    => c,
            Some(&Err(err)) => return Some(Err(err)),
            None            => return None,
        };

        let output = match self.state {
            State::ExpectingFirstInteger     if is_digit(c)              => {
                self.read_integer(State::ExpectingX)
            },
            State::ExpectingSecondInteger    if is_digit(c)              => {
                self.read_integer(State::ExpectingCloseParenthesis)
            },
            State::ExpectingFirstInteger | State::ExpectingSecondInteger => {
                self.error(DecompressError::ExpectedInteger)
            },
            State::ExpectingX                if c == 'x'                 => {
                self.token(c, State::ExpectingSecondInteger, DecompressTokenType::X)
            },
            State::ExpectingX                                            => {
                self.error(DecompressError::ExpectedX)
            },
            State::ExpectingCloseParenthesis if c == ')'                 => {
                self.token(c, State::Initial, DecompressTokenType::CloseParenthesis)
            },
            State::ExpectingCloseParenthesis                             => {
                self.error(DecompressError::ExpectedCloseParenthesis)
            },
            State::Initial if c == '('                                   => {
                self.token(c, State::ExpectingFirstInteger, DecompressTokenType::OpenParenthesis)
            },
            State::Initial                                               => {
                self.token(c, State::Initial, DecompressTokenType::Character(c))
            },
            State::Error                                                 => {
                return None;
            },
        };

        Some(output)
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn read_integer(chars: &mut Peekable<FileChars>) -> Result<(Vec<char>, usize), DecompressError> {
    let unparsed_integer: String = match take_while(chars, |l, c| l <= 10 && is_digit(c)) {
        Ok(v)    => v.into_iter().collect(),
        Err(err) => return Err(err),
    };

    match unparsed_integer.parse::<usize>() {
        Ok(i)  => Ok((unparsed_integer.chars().collect(), i)),
        Err(_) => Err(DecompressError::CouldNotParseInteger),
    }
}

fn take_while<F>(chars: &mut Peekable<FileChars>, condition: F) -> Result<Vec<char>, DecompressError>
    where F : Fn(usize, char) -> bool {

    let mut output = Vec::new();

    while let Some(&c) = chars.peek() {
        match c {
            Ok(c) if condition(output.len(), c) => {
                output.push(c);
                chars.next();
            },
            Ok(_)                   => {
                break;
            }
            Err(err)                => {
                return Err(err);
            },
        };
    }

    Ok(output)
}
