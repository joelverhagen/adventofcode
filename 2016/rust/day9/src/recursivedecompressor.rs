use std::io;
use decompresserror::DecompressError;
use decompresstokens::DecompressTokens;
use decompresstokens::DecompressToken;
use decompresstokens::DecompressTokenType;

pub struct RecursiveDecompressor {
    tokens: DecompressTokens,
    repeat_sequences: Vec<RepeatSequence>,
    sum: u64,
}

#[derive(Debug)]
struct RepeatSequence {
    repeat_length: usize,
    repeat_count: usize,
    repeat_length_remaining: usize,
    text_length: usize,
    sum: u64,
}

impl RecursiveDecompressor {
    pub fn open(path: &str) -> Result<RecursiveDecompressor, io::Error> {
        let tokens = match DecompressTokens::open(path) {
            Ok(tokens) => tokens,
            Err(err)   => return Err(err),
        };

        Ok(RecursiveDecompressor {
            tokens: tokens,
            repeat_sequences: Vec::new(),
            sum: 0,
        })
    }

    fn consume_repeat_sequence(&mut self, t0_length: usize) -> Result<RepeatSequence, DecompressError> {
        match (self.tokens.next(),
               self.tokens.next(),
               self.tokens.next(),
               self.tokens.next()) {
              (Some(Ok(DecompressToken { text: t1, token_type: DecompressTokenType::Integer(length) })),
               Some(Ok(DecompressToken { text: t2, token_type: DecompressTokenType::X })),
               Some(Ok(DecompressToken { text: t3, token_type: DecompressTokenType::Integer(count) })),
               Some(Ok(DecompressToken { text: t4, token_type: DecompressTokenType::CloseParenthesis }))) => {
                Ok(RepeatSequence {
                    text_length: t0_length + t1.len() + t2.len() + t3.len() + t4.len(),
                    repeat_length: length,
                    repeat_count: count,
                    repeat_length_remaining: length,
                    sum: 0,
                })
              },
              _                                   => {
                Err(DecompressError::InvalidRepeatDirective)
              },
        }
    }

    fn pop_and_add(&mut self) {
        while self.repeat_sequences.len() > 0 {
            if self.repeat_sequences.last().unwrap().repeat_length_remaining > 0 {
                return;
            }

            let completed = self.repeat_sequences.pop().unwrap();
            let total = completed.sum * (completed.repeat_count as u64);
            self.add_value(total);
        }
    }

    fn add_value(&mut self, value: u64) {
        match self.repeat_sequences.last_mut() {
            Some(last) => last.sum += value,
            None       => self.sum += value,
        };
    }

    fn subtract_text_length(&mut self, length: usize) {
        for repeat_sequence in self.repeat_sequences.iter_mut() {
            repeat_sequence.repeat_length_remaining -= length;
        }
    }

    pub fn len(mut self) -> Result<u64, DecompressError> {
        loop {
            let token = match self.tokens.next() {
                Some(Ok(token)) => token,
                Some(Err(err))  => return Err(err),
                None            => break,
            };

            match token {
                DecompressToken { text: t0, token_type: DecompressTokenType::OpenParenthesis } => {
                    let repeat_sequence = match self.consume_repeat_sequence(t0.len()) {
                        Ok(repeat_sequence) => repeat_sequence,
                        Err(err)            => return Err(err),
                    };

                    self.subtract_text_length(repeat_sequence.text_length);

                    self.repeat_sequences.push(repeat_sequence);
                },
                DecompressToken { text, token_type: _ }                                        => {
                    self.add_value(text.len() as u64);

                    self.subtract_text_length(text.len());

                    let should_pop = match self.repeat_sequences.last() {
                        Some(s) if s.repeat_length_remaining == 0 => true,
                        _                                         => false,
                    };

                    if should_pop {
                        self.pop_and_add();
                    }
                },
            }
        }

        self.pop_and_add();

        Ok(self.sum)
    }
}
