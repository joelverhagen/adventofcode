use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum RoomParseError {
    CouldNotOpenRoomFile,
    NotEnoughHyphenPieces(usize),
    WrongNumberOfLeftSquareBracketPieces(usize),
    SectorIdCouldNotBeParsed(String),
    ChecksumMustHaveFiveChars(usize)
}

#[derive(Debug)]
struct Room {
    pieces: Vec<Vec<char>>,
    sector_id: i32,
    checksum: Vec<char>,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut char_counts: HashMap<char, i32> = HashMap::new();

        for piece in &self.pieces {
            for c in piece {
                let entry = char_counts.entry(*c).or_insert(0);
                *entry += 1;
            }
        }

        let mut char_counts: Vec<(&char, &i32)> = char_counts
            .iter()
            .collect();

        char_counts.sort_by(|a, b| {
            match a.1.cmp(b.1).reverse() { 
                Ordering::Equal => a.0.cmp(b.0),
                other           => other,
            }
        });

        let char_order: Vec<char> = char_counts
            .iter()
            .map(|a| *a.0)
            .collect();

        for i in 0..5 {
            if char_order[i] != self.checksum[i] {
                return false
            }
        }

        true
    }

    fn parse(unparsed_room: &str) -> Result<Room, RoomParseError> {
        let mut pieces: Vec<&str> = unparsed_room
            .split('-')
            .collect();

        if pieces.len() < 2 {
            return Err(RoomParseError::NotEnoughHyphenPieces(pieces.len()))
        }

        let last_piece = pieces.pop().unwrap();

        let pieces: Vec<Vec<char>> = pieces
            .iter()
            .map(|&p| p.chars().collect())
            .collect();

        let sector_id_and_checksum: Vec<&str> = last_piece
            .split('[')
            .collect();

        if sector_id_and_checksum.len() != 2 {
            return Err(RoomParseError::WrongNumberOfLeftSquareBracketPieces(sector_id_and_checksum.len()))
        }

        let sector_id_result = sector_id_and_checksum[0].parse::<i32>();
        if sector_id_result.is_err() {
            return Err(RoomParseError::SectorIdCouldNotBeParsed(sector_id_and_checksum[0].to_string()))
        }

        let checksum: Vec<char> = sector_id_and_checksum[1]
            .trim_right_matches(']')
            .chars()
            .collect();

        if checksum.len() != 5 {
            return Err(RoomParseError::ChecksumMustHaveFiveChars(checksum.len()))
        }

        Ok(Room {
            pieces: pieces,
            sector_id: sector_id_result.unwrap(),
            checksum: checksum,
        })
    }
}

fn parse_room_file(path: &str) -> Result<Vec<Room>, RoomParseError> {
    let file = match File::open(path) {
        Err(_)   => return Err(RoomParseError::CouldNotOpenRoomFile),
        Ok(file) => file,
    };

    let file_reader = BufReader::new(file);
    let mut rooms: Vec<Room> = Vec::new();

    for line_result in file_reader.lines() {
        let line = match line_result {
            Err(_)   => return Err(RoomParseError::CouldNotOpenRoomFile),
            Ok(line) => line,
        };

        let room = Room::parse(&line)?;
        rooms.push(room);
    }

    Ok(rooms)
}

fn get_real_room_sector_id_sum(rooms: &Vec<Room>) -> i32 {
    let mut sum = 0;
    for room in rooms {
        if room.is_real() {
            sum += room.sector_id;
        }
    }

    sum
}

fn main() {
    let rooms = parse_room_file("input.txt").unwrap();

    let part_1_result = get_real_room_sector_id_sum(&rooms);
    println!("Part 1 result: {}", part_1_result);
}
    