use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
enum IpAddressParseError {
    CouldNotReadFile,
}

#[derive(Debug)]
struct IpAddress {
    sequences: Vec<IpAddressSequence>,
}

#[derive(Debug)]
struct IpAddressSequence {
    is_hypernet: bool,
    chars: Vec<char>,
}

impl IpAddressSequence {
    fn maybe_push_new(is_hypernet: bool, chars: Vec<char>, sequences: &mut Vec<IpAddressSequence>) {
        if chars.len() > 0 {
            sequences.push(IpAddressSequence {
                is_hypernet: is_hypernet,
                chars: chars,
            });
        }
    }

    fn has_abba(&self) -> bool {
        if self.chars.len() < 4 {
            return false;
        }

        for i in 3..self.chars.len() {
            if self.chars[i - 3] == self.chars[i] &&
               self.chars[i - 2] == self.chars[i - 1] &&
               self.chars[i - 3] != self.chars[i - 2]
            {
                return true;
            }
        }

        false
    }
}

impl IpAddress {
    fn parse(unparsed: &str) -> IpAddress {
        let mut sequences: Vec<IpAddressSequence> = Vec::new();
        let mut chars: Vec<char> = Vec::new();
        let mut is_hypernet = false;

        for c in unparsed.chars() {
            let is_sequence_complete = match c {
                '[' => true,
                ']' => { is_hypernet = true; true },
                _   => false,
            };

            if is_sequence_complete {
                IpAddressSequence::maybe_push_new(is_hypernet, chars, &mut sequences);
                chars = Vec::new();
                is_hypernet = false;
            } else {
                chars.push(c);
            }
        }
        
        IpAddressSequence::maybe_push_new(is_hypernet, chars, &mut sequences);

        IpAddress {
            sequences: sequences
        }
    }

    fn supports_tls(&self) -> bool {
        let mut has_abba = false;

        for sequence in &self.sequences {
            if sequence.has_abba() {
                has_abba = true;

                if sequence.is_hypernet {
                    return false;
                }                
            } 
        }

        has_abba
    }
}

fn parse_ip_address_file(path: &str) -> Result<Vec<IpAddress>, IpAddressParseError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_)   => return Err(IpAddressParseError::CouldNotReadFile)
    };

    let file_reader = BufReader::new(file);
    let mut ip_addresses: Vec<IpAddress> = Vec::new();

    for line_result in file_reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_)   => return Err(IpAddressParseError::CouldNotReadFile),
        };

        ip_addresses.push(IpAddress::parse(&line));
    }

    Ok(ip_addresses)
}

fn count_supporting_tls(ip_addresses: &Vec<IpAddress>) -> usize {
    ip_addresses
        .iter()
        .filter(|i| i.supports_tls())
        .count()
}

fn main() {
    let ip_addresses = parse_ip_address_file("input.txt").unwrap();

    let part_1_result = count_supporting_tls(&ip_addresses);
    println!("Part 1 result: {}", part_1_result);
}
