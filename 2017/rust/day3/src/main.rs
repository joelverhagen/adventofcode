use std::collections::HashMap;

fn fill_adjacent_sums_up_to(input: i32) -> (HashMap<(i32, i32), i32>, (i32, i32)) {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let mut last_sum = 1;
    grid.insert((0, 0), last_sum);

    let mut side_steps_x_2 = 2;
    let mut h = 0;
    let mut v = 0;
    let directions = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let adjacent_offsets = vec![(1, 0), (0, -1), (-1, 0), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];

    'outer: loop {
        for &(h_dir, v_dir) in &directions {
            for _ in 0..side_steps_x_2 / 2 {
                h += h_dir;
                v += v_dir;

                let mut sum = 0;
                for adjacent_offset in &adjacent_offsets {
                    let key = (h + adjacent_offset.0, v + adjacent_offset.1);
                    sum += match grid.get(&key) {
                        Some(s) => *s,
                        None    => 0,
                    };
                }

                last_sum = sum;
                grid.insert((h, v), sum);
                if last_sum > input {
                    break 'outer;
                }
            }

            side_steps_x_2 += 1;
        }
    }

    (grid, (h, v))
}

fn find_frst_sum_after(input: i32) -> i32 {
    let (grid, last) = fill_adjacent_sums_up_to(input);

    *grid.get(&last).expect("Could not find last sum.")
}

fn adjust_until(h: &mut i32, v: &mut i32, h_dir: i32, v_dir: i32, side_steps: i32, current: &mut i32, input: i32) -> bool {
    if *current == input {
        return true;
    }

    // When moving left or up, subtract then add.
    // When moving right or down, add then subtract.
    let first_sign = if h_dir < 0 || v_dir < 0 { -1 } else { 1 };

    for i in 0..side_steps {
        if i < side_steps / 2 {
            *h -= first_sign * h_dir;
            *v -= first_sign * v_dir;
        } else {
            *h += first_sign * h_dir;
            *v += first_sign * v_dir;
        }

        *current -= 1;

        if *current == input {
            return true;
        }
    }

    return false;
}

fn find_root_of_next_odd_square(input: i32) -> i32 {
    let mut odd = 1;
    let mut square = 1;
    while input > square {
        odd += 2;
        square = odd * odd;
    }

    odd
}

fn spiral_manhattan(input: i32) -> i32 {
    let odd = find_root_of_next_odd_square(input);
    let square = odd * odd;

    let side_steps = odd - 1;
    let mut h = odd / 2;
    let mut v = h;
    let mut current = square;

    if !adjust_until(&mut h, &mut v, -1, 0, side_steps, &mut current, input) {
        if !adjust_until(&mut h, &mut v, 0, -1, side_steps, &mut current, input) {
            if !adjust_until(&mut h, &mut v, 1, 0, side_steps, &mut current, input) {
                if !adjust_until(&mut h, &mut v, 0, 1, side_steps, &mut current, input) {
                    panic!("The input value was not found in the ring.");
                }
            }
        }
    }

    h + v
}

fn main() {
    let input = 325489;
    println!("Day 3, part 1: {}", spiral_manhattan(input));
    println!("Day 3, part 2: {}", find_frst_sum_after(input));
}
