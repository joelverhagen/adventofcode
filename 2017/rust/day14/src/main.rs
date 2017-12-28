extern crate adventofcode;
use std::collections::HashSet;
use std::fmt::Write;
use adventofcode::day10::knot_hash;

fn generate_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for i in 0..128 {
        let mut row_input = String::new();
        write!(&mut row_input, "{}-{}", input, i).unwrap();
        let byte_input = row_input.chars().map(|c| c as u32).collect();

        let hash = knot_hash(&byte_input);
        grid.push(hash);
    }

    grid
}

fn byte_to_binary(byte: u8) -> Vec<bool> {
    let mut binary_string = String::new();
    write!(&mut binary_string, "{:08b}", byte).unwrap();
    
    binary_string
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn count_regions(grid: &Vec<Vec<u8>>) -> u32 {
    // Expand the grid of bytes to a grid of booleans, for simplicity.
    let binary_grid: Vec<Vec<bool>> = grid
        .iter()
        .map(|r| r
            .iter()
            .flat_map(|b| byte_to_binary(*b))
            .collect())
        .collect();

    let mut group_count = 0;
    let mut neighbors = vec![(0, 0)];
    let offsets: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    
    while neighbors.len() > 0 {
        let next_neighbor = neighbors.pop().unwrap();
        let group_value = binary_grid[next_neighbor.0][next_neighbor.1];

        if visited.contains(&next_neighbor) {
            continue;
        }

        if group_value {
            group_count += 1;
        }

        let mut to_visit = vec![next_neighbor];
        while to_visit.len() > 0 {
            let current = to_visit.pop().unwrap();

            if !visited.insert(current) {
                continue;
            }

            for offset in &offsets {
                let ineighbor = (current.0 as isize + offset.0, current.1 as isize + offset.1);

                // Check bounds before inspecting the neighbor.
                if ineighbor.0 < 0
                   || ineighbor.1 < 0
                   || ineighbor.0 as usize >= binary_grid.len()
                   || ineighbor.1 as usize >= binary_grid[ineighbor.0 as usize].len() {
                    continue;
                }

                let neighbor = (ineighbor.0 as usize, ineighbor.1 as usize);
                let value = binary_grid[neighbor.0][neighbor.1];
                if value == group_value {
                    to_visit.push(neighbor);                  
                } else {
                    neighbors.push(neighbor);
                }
            }
        }
    }

    group_count
}

fn count_ones(grid: &Vec<Vec<u8>>) -> u32 {
    grid
        .iter()
        .map(|r| r
            .iter()
            .map(|b| b.count_ones())
            .sum::<u32>())
        .sum()
}

fn main() {
    let key = "vbqugkhl";
    let grid = generate_grid(key);
    println!("Day 14, part 1: {}", count_ones(&grid));
    println!("Day 14, part 2: {}", count_regions(&grid));
}
