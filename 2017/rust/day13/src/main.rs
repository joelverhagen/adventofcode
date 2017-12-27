use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct FirewallLayer {
    depth: i32,
    range: i32,
    position: i32,
    forward: bool,
}

impl FirewallLayer {
    fn step(&mut self) {
        if self.range < 2 {
            return;
        }

        self.position += if self.forward { 1 } else { -1 };

        if self.position >= self.range {
            self.position = self.range - 2;
            self.forward = false;
        } else if self.position < 0 {
            self.position = 1;
            self.forward = true;
        }
    }
}

#[derive(Clone, Debug)]
struct Layers {
    items: Vec<FirewallLayer>,
    max_depth: i32,
}

#[derive(Clone, Debug)]
struct Packet {
    depth: i32,
    position: i32,
}

#[derive(Clone, Debug)]
struct State {
    packet: Packet,
    layers: Layers,
}

impl State {
    fn step_layers(&mut self) {
        for layer in &mut self.layers.items {
            layer.step();
        }
    }

    fn step(&mut self, only_first: bool) -> Option<i32> {
        let mut severity = None;

        // 1) Move the packet forward.
        self.packet.depth += 1;

        // 2) Have scanners found with the packet?
        for layer in &self.layers.items {
            if self.packet.depth == layer.depth && self.packet.position == layer.position {
                severity = Some(severity.unwrap_or(0) + (layer.depth * layer.range));

                if severity.is_some() && only_first {
                    return severity;
                }
            }
        }

        // 3) Move the scanners forward.
        self.step_layers();

        severity
    }
}

fn parse_layer(line: &str) -> FirewallLayer {
    let pieces: Vec<&str> = line
        .split(|c: char| c == ':' || c.is_whitespace())
        .filter(|&p| p.len() > 0)
        .collect();
    let depth = pieces[0].parse::<i32>().expect("Could not parse the firewall depth as i32.");
    let range = pieces[1].parse::<i32>().expect("Could not parse the firewall range as i32.");
    
    FirewallLayer {
        depth,
        range,
        position: 0,
        forward: true,
    }
}

fn read_layers(path: &str) -> Layers {
    let f = File::open(path).expect("Could not open the specified file.");

    let items: Vec<FirewallLayer> = BufReader::new(f)
        .lines()
        .map(|lr| lr.expect("Could not read a line."))
        .map(|l| parse_layer(&l))
        .collect();

    let max_depth = items
        .iter()
        .map(|x| x.depth)
        .max()
        .unwrap_or(0);

    Layers {
        items,
        max_depth,
    }
}

fn get_severity(state: &mut State, only_first: bool) -> Option<i32> {
    let mut severity = None;

    while state.packet.depth < state.layers.max_depth {
        severity = match (severity, state.step(only_first)) {
            (Some(a), Some(b)) => Some(a + b),
            (None,    Some(b)) => Some(b),
            (Some(a), None   ) => Some(a),
            (None,    None   ) => None,
        };

        if severity.is_some() && only_first {
            break;
        }
    }

    severity
}

fn read_state(path: &str) -> State {
    let packet = Packet { depth: -1, position: 0, };
    let layers = read_layers(path);
    
    State {
        packet,
        layers,
    }
}

fn simulate_part_1(path: &str) -> Option<i32> {
    let mut state = read_state(path);

    get_severity(&mut state, false)
}

fn simulate_part_2(path: &str) -> i32 {
    let mut wait_steps = 0;
    let mut initial_state = read_state(path);

    loop {
        let mut state = initial_state.clone();
        let severity = get_severity(&mut state, true);

        if severity.is_none() {
            break;
        }

        initial_state.step_layers();
        wait_steps += 1;
    }

    wait_steps
}

fn main() {
    let path = "input.txt";
    println!("Day 13, part 1: {:?}", simulate_part_1(path));
    println!("Day 13, part 2: {}", simulate_part_2(path));
}
