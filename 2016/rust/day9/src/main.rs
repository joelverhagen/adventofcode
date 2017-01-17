mod decompresserror;
mod filechars;
mod decompresstokens;
mod decompressor;
mod recursivedecompressor;

use decompressor::Decompressor;
use recursivedecompressor::RecursiveDecompressor;

fn main() {
    let path = "input.txt";

    let part_1_result = Decompressor::open(path)
        .unwrap()
        .len()
        .unwrap();
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = RecursiveDecompressor::open(path)
        .unwrap()
        .len()
        .unwrap();
    println!("Part 2 result: {}", part_2_result);
}
