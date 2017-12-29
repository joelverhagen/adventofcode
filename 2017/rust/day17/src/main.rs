#[derive(Debug)]
struct Spinlock {
    steps: usize,
    index: usize,
    buffer: Vec<usize>,
    value: usize,
}

impl Spinlock {
    fn new(steps: usize) -> Spinlock {
        let mut buffer = Vec::new();
        buffer.push(0);

        Spinlock {
            steps: steps,
            index: 0,
            buffer,
            value: 1,
        }
    }

    fn step(&mut self) {
        self.index = ((self.index + self.steps) % self.buffer.len()) + 1;
        self.buffer.insert(self.index, self.value);
        self.value += 1;
    }
}

fn get_part_1(steps: usize) -> usize {
    let mut sl = Spinlock::new(steps);
    for _ in 0..2017 {
        sl.step();
    }

    sl.buffer[sl.index + 1]
}


fn main() {
    println!("Day 17, part 1: {}", get_part_1(377));
}
