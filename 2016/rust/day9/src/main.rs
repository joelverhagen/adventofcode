mod decompresserror;
mod filechars;
mod decompresstokens;
mod decompressor;

use decompressor::Decompressor;

fn main() {
    let part_1_result = Decompressor::open("input.txt")
        .unwrap()
        .read_to_end()
        .unwrap()
        .len();
    println!("Part 1 result: {}", part_1_result);
}
