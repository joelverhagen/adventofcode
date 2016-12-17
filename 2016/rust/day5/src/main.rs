extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_hex_char(b: u8) -> Option<char> {
    match b {
        0  ...  9 => Some((b + ('0' as u8)) as char),
        10 ... 15 => Some(((b - 10) + ('a' as u8)) as char),
        _         => None
    }
}

fn get_password_char(hasher: &mut Md5, door_id_bytes: &[u8], index: u64) -> Option<char> {
    hasher.input(&door_id_bytes);
    hasher.input(index.to_string().as_bytes());

    let mut hash = [0; 16];
    hasher.result(&mut hash);
    hasher.reset();

    match (hash[0], hash[1], hash[2]) {
        (0, 0, b) if b < 16 => get_hex_char(b),
        _                   => None,
    }
}

fn get_password(door_id: &str) -> String {
    let door_id_bytes = door_id.as_bytes();
    let mut password = String::new();
    let mut hasher = Md5::new();

    for index in 0.. {
        match get_password_char(&mut hasher, &door_id_bytes, index) {
            Some(c) => password.push(c),
            None    => continue,
        };

        // println!("{}{} -> {:?}", door_id, index, password);

        if password.len() >= 8 {
            break;
        }
    }

    password
}

fn main() {
    let day_1_result = get_password("uqwqemis");
    println!("Part 1 result: {}", day_1_result);
}
