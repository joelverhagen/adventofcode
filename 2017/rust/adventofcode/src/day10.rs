pub fn evaluate_lengths(list: &mut Vec<u8>, lengths: &Vec<u32>, initial_position: u32, initial_skip_size: u32) -> (u32, u32) {
    let list_size = list.len() as u32;
    let mut position = initial_position;
    let mut skip_size = initial_skip_size;

    for length in lengths {
        if *length > list_size {
            continue;
        }

        for i in 0..length / 2 {
            let from_index = ((position + i) % list_size) as usize;
            let to_index = ((position + (length - i) - 1) % list_size) as usize;
            let temp = list[to_index];
            list[to_index] = list[from_index];
            list[from_index] = temp;
        }

        position = (position + length + skip_size) % list_size;
        skip_size += 1;
    }

    (position, skip_size)
}

pub fn knot_hash(input: &Vec<u32>) -> Vec<u8> {
    let mut sparse_hash: Vec<u8> = (0..256 as u16).map(|b| b as u8).collect();
    let mut lengths = input.clone();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        let (next_position, next_skip_size) = evaluate_lengths(&mut sparse_hash, &lengths, position, skip_size);
        position = next_position;
        skip_size = next_skip_size;
    }

    let mut dense_hash = Vec::new();
    for i in 0..sparse_hash.len() / 16 {
        let starting_index = i * 16;
        let mut current = sparse_hash[starting_index];
        for offset in 1..16 {
            let index = starting_index + offset;
            current ^= sparse_hash[index];
        }

        dense_hash.push(current);
    }

    dense_hash
}