use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use facilitystate::FacilityState;
use facilitystate::Step;
use facilitystate::Direction;
use floor::FacilityObject;
use floor::FloorParseError;
use helpers::dedup;

pub struct Solver {
    all_states: HashMap<Rc<FacilityState>, ()>,
    initial_state: Rc<FacilityState>,
}

impl Solver {
    pub fn solve(&mut self) {
        let mut step_queue: VecDeque<(Option<Step>, Rc<FacilityState>)> = VecDeque::new();
        let mut step_map: HashMap<(Option<Step>, Rc<FacilityState>), Step> = HashMap::new();
        step_queue.push_back((None, self.initial_state.clone()));

        while step_queue.len() > 0 {
            let (previous_step, state) = step_queue.pop_front().unwrap();
            let next_steps = self.enumerate_steps(&state);

            // println!("{:#?}", next_steps);

            for (step, next_state) in next_steps {
                if next_state.is_complete() {
                    println!("{}", next_state.display());
                    return;
                }

                step_queue.push_back((Some(step), next_state));
            }
        }
    }

    pub fn parse(path: &str) -> Result<Solver, FloorParseError> {
        let mut all_elements: HashMap<Rc<String>, ()> = HashMap::new();
        let mut all_objects: HashMap<Rc<FacilityObject>, ()> = HashMap::new();
        let initial_state = FacilityState::parse(path, &mut all_elements, &mut all_objects)?;

        let mut all_states: HashMap<Rc<FacilityState>, ()> = HashMap::new();
        let initial_state = dedup(&mut all_states, initial_state);

        println!("{}", initial_state.display());

        Ok(Solver {
            all_states: all_states,
            initial_state: initial_state,
        })
    }

    fn enumerate_steps(&mut self, state: &FacilityState) -> Vec<(Step, Rc<FacilityState>)> {
        let current_floor = state.current_floor();
        let floor_objects = state
            .floors()
            .get(current_floor - 1)
            .unwrap()
            .objects()
            .iter()
            .cloned()
            .collect();

        let mut all_steps = Vec::new();

        // Determine which directions we can go.
        let directions = match current_floor {
            cf if cf == 1                    => vec![Direction::Up],
            cf if cf == state.floors().len() => vec![Direction::Down],
            _                                => vec![Direction::Up, Direction::Down],
        };

        // Determine possible facility objects we can move.
        for direction in directions {
            let combinations = get_combinations(&floor_objects, 2);
            for objects in combinations {
                let objects = objects
                    .into_iter()
                    .collect();
                all_steps.push(Step::new(direction, objects));
            }
        }

        // Validate the generated steps.
        let mut valid_steps = Vec::new();
        for step in all_steps {
            let new_state = match state.apply(&step) {
                Ok(new_state) => new_state,
                Err(_)        => continue,
            };

            // Determine if we've been to this state before.
            if self.all_states.contains_key(&new_state) {
                continue;
            } else {
                let new_state = dedup(&mut self.all_states, new_state);
                valid_steps.push((step, new_state));
            }
        }

        valid_steps
    }
}

fn get_combinations<T: Clone>(items: &Vec<T>, up_to_count: usize) -> Vec<Vec<T>> {
    let mut output = Vec::new();

    if up_to_count > 1 {
        // add combinations containing two items
        for i in 0..items.len() {
            for j in (i + 1)..items.len() {
                output.push(vec![
                    items[i].clone(),
                    items[j].clone(),
                ])
            }
        }
    }

    if up_to_count > 0 {
        // add combinations containing one item
        for item in items {
            output.push(vec![item.clone()]);
        }        
    }

    output
}
