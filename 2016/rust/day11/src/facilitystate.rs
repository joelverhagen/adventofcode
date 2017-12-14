use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;
use floor::Floor;
use floor::FacilityObject;
use floor::FloorParseError;

#[derive(Debug)]
pub enum FacilityStateError {
    InvalidStepTransition,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Step {
    direction: Direction,
    objects: Vec<Rc<FacilityObject>>,
}

impl Step {
    pub fn new(direction: Direction, objects: Vec<Rc<FacilityObject>>) -> Step {
        Step {
            direction: direction,
            objects: objects,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FacilityState {
    floors: Vec<Floor>,
    current_floor: usize,
}

#[derive(Debug)]
struct Elements { g: bool, m: bool, }

impl FacilityState {
    pub fn current_floor(&self) -> usize {
        self.current_floor
    }

    pub fn floors(&self) -> &Vec<Floor> {
        &self.floors
    }

    pub fn is_complete(&self) -> bool {
        self
            .floors
            .iter()
            .take(self.floors().len() - 1)
            .all(|f| f.objects().len() == 0)
    }

    pub fn apply(&self, step: &Step) -> Result<FacilityState, FacilityStateError> {
        // Move the elevator.
        let next_floor_number = match (step.direction, self.current_floor) {
            (Direction::Down, cf) if cf == 1                 => return Err(FacilityStateError::InvalidStepTransition),
            (Direction::Up,   cf) if cf == self.floors.len() => return Err(FacilityStateError::InvalidStepTransition),
            (Direction::Down, _ )                            => self.current_floor - 1,
            (Direction::Up,   _ )                            => self.current_floor + 1,
        };

        // Build the new list of floors.
        let mut next_floors: Vec<Floor> = Vec::new();
        for floor in &self.floors {
            let new_floor = match floor.number() {
                n if n == self.current_floor => floor.remove_objects(&step.objects),
                n if n == next_floor_number  => floor.add_objects(&step.objects),
                _                            => floor.clone(),
            };

            if floor.number() == self.current_floor || floor.number() == next_floor_number {
                // Validate the new floor.
                let mut elements: HashMap<Rc<String>, Elements> = HashMap::new();
                let mut has_g = false;
                for object in new_floor.objects() {
                    let entry = elements
                        .entry(object.element())
                        .or_insert(Elements { g: false, m: false, });

                    match **object {
                        FacilityObject::Microchip(_) => { entry.m = true; },
                        FacilityObject::Generator(_) => { entry.g = true; has_g = true },
                    };
                }

                for element in &elements {
                    match element {
                        (_, &Elements { g: false, m: true }) if has_g => return Err(FacilityStateError::InvalidStepTransition),
                        _                                             => {},
                    };
                }
            }

            next_floors.push(new_floor);
        }

        Ok(FacilityState {
            floors: next_floors,
            current_floor: next_floor_number,
        })
    }

    pub fn parse(
        path: &str,
        all_elements: &mut HashMap<Rc<String>, ()>,
        all_objects: &mut HashMap<Rc<FacilityObject>, ()>) -> Result<FacilityState, FloorParseError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(FloorParseError::CouldNotOpenFile(err)),
        };

        let reader = BufReader::new(file);
        let mut floors: Vec<Floor> = Vec::new();
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(line) => line,
                Err(err) => return Err(FloorParseError::CouldNotReadFile(err)),
            };

            let floor = Floor::parse(all_elements, all_objects, &line)?;
            floors.push(floor);
        }

        floors.sort_by(|a, b| a.number().cmp(&b.number()));

        Ok(FacilityState {
            floors: floors,
            current_floor: 1,
        })
    }

    fn unique_element_prefix_length(&self) -> usize {
        let unique_elements = self
            .floors
            .iter()
            .flat_map(|f| f
                .objects()
                .iter()
                .map(|o| o.element().to_string().to_uppercase()))
            .collect::<HashSet<String>>();
        
        get_unique_prefix_length(&unique_elements)
    }

    pub fn display(&self) -> String {
        let prefix_length = self.unique_element_prefix_length();
        let padding = (0..prefix_length)
            .map(|_| ' ')
            .collect::<String>();

        // Generate and sort the object labels.
        let mut object_labels: Vec<(usize, String)> = self
            .floors
            .iter()
            .flat_map(|f| f
                .objects()
                .iter()
                .map(|o| (f.number(), o.get_label(prefix_length)))
                .collect::<Vec<(usize, String)>>())
            .collect();

        object_labels.sort_by(|a, b| a.1.cmp(&b.1));

        // Group the labels by floor.
        let mut floor_groups: HashMap<usize, VecDeque<(usize, String)>> = HashMap::new();
        let mut object_index = 0;
        for (number, object) in object_labels {
            let objects = floor_groups
                .entry(number)
                .or_insert(VecDeque::new());

            objects.push_back((object_index, object));
            object_index += 1;
        }

        // Write each floor.
        let mut output = String::new();
        for floor in self.floors.iter().rev() {
            let number = floor.number();

            // Write the floor number.
            write!(&mut output, "F{}{}", number, padding).unwrap();

            // Write the elevator position.
            if self.current_floor == number {
                write!(&mut output, "E {}", padding).unwrap();
            } else {
                write!(&mut output, ". {}", padding).unwrap();
            }

            // Write the objects.
            let objects = floor_groups
                .entry(number)
                .or_insert(VecDeque::new());

            for i in 0..object_index {
                let should_pop = match objects.front() {
                    Some(&(object_index, _)) if object_index == i => true,
                    Some(&(_, _))                                 => false,
                    None                                          => false,
                };

                if should_pop {
                    let (_, object) = objects.pop_front().unwrap();
                    write!(&mut output, "{} ", object).unwrap();    
                } else {
                    write!(&mut output, ". {}", padding).unwrap();
                }
            }

            if number > 1 {
                writeln!(&mut output, "").unwrap();
            }
        }

        output
    }
}

fn get_unique_prefix_length(values: &HashSet<String>) -> usize {
    let mut length = 1;
    let mut prefixes: HashSet<String> = HashSet::new();
    let mut remaining_values = values
        .iter()
        .collect::<Vec<&String>>();

    while remaining_values.len() > 0 {
        let prefix = remaining_values
            .pop()
            .unwrap()
            .chars()
            .take(length)
            .collect::<String>();

        if prefixes.contains(&prefix) {
            length += 1;
            prefixes.clear();
            remaining_values = values
                .iter()
                .collect::<Vec<&String>>();
        } else {
            prefixes.insert(prefix);
        }
    }

    length
}