use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::io;
use std::rc::Rc;
use regex::Regex;
use helpers::dedup;

#[derive(Debug)]
pub enum FloorParseError {
    CouldNotOpenFile(io::Error),
    CouldNotReadFile(io::Error),
    NoFloorNumberFound,
    UnrecognizedFloorNumber,
    UnrecognizedObject,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FacilityObject {
    Microchip(Rc<String>),
    Generator(Rc<String>),
}

impl FacilityObject {
    pub fn element(&self) -> Rc<String> {
        match self {
            &FacilityObject::Microchip(ref element) => element.clone(),
            &FacilityObject::Generator(ref element) => element.clone(),
        }
    }

    pub fn get_label(&self, chars: usize) -> String {
        let pieces = match self {
            &FacilityObject::Microchip(ref element) => ('M', element.chars()),
            &FacilityObject::Generator(ref element) => ('G', element.chars()),
        };

        let mut output = String::new();
        write!(
            &mut output,
            "{}{}",
            pieces.1.take(chars).collect::<String>().to_uppercase(),
            pieces.0).unwrap();

        output
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Floor {
    number: usize,
    objects: Vec<Rc<FacilityObject>>,
}

impl Clone for Floor {
    fn clone(&self) -> Floor {
        let new_objects = self
            .objects
            .iter()
            .cloned()
            .collect();

        Floor {
            number: self.number,
            objects: new_objects,
        }
    }
}

impl Floor {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn objects(&self) -> &Vec<Rc<FacilityObject>> {
        &self.objects
    }

    pub fn remove_objects(&self, remove: &Vec<Rc<FacilityObject>>) -> Floor {
        let new_objects = self
            .objects
            .iter()
            .cloned()
            .collect::<HashSet<Rc<FacilityObject>>>()
            .difference(&remove
                .iter()
                .cloned()
                .collect())
            .cloned()
            .collect::<Vec<Rc<FacilityObject>>>();

        Floor {
            number: self.number,
            objects: new_objects,
        }
    }

    pub fn add_objects(&self, add: &Vec<Rc<FacilityObject>>) -> Floor {
        let new_objects = self
            .objects
            .iter()
            .cloned()
            .collect::<HashSet<Rc<FacilityObject>>>()
            .union(&add
                .iter()
                .cloned()
                .collect())
            .cloned()
            .collect::<Vec<Rc<FacilityObject>>>();

        Floor {
            number: self.number,
            objects: new_objects,
        }
    }

    pub fn parse(
        all_elements: &mut HashMap<Rc<String>, ()>,
        all_objects: &mut HashMap<Rc<FacilityObject>, ()>,
        input: &str) -> Result<Floor, FloorParseError> {
        
        lazy_static! {
            static ref FLOOR: Regex = Regex::new("^The (?P<floor>\\w+) floor").unwrap();
            static ref OBJECT: Regex = Regex::new("an? (?P<element>\\w+)(?P<type>-compatible microchip| generator)").unwrap();
        }

        let unparsed_number = match FLOOR.captures(input) {
            Some(caps) => caps.get(1).unwrap().as_str(),
            None       => return Err(FloorParseError::NoFloorNumberFound),
        };
        let number = parse_floor_number(unparsed_number)?;

        let mut objects: Vec<Rc<FacilityObject>> = Vec::new();

        for caps in OBJECT.captures_iter(input) {
            let element = caps["element"].to_string();
            let element_rc = dedup(all_elements, element);

            let object = parse_object(element_rc, &caps["type"])?;
            let object_rc = dedup(all_objects, object);
            objects.push(object_rc);
        }

        Ok(Floor {
            number: number,
            objects: objects,
        })
    }
}

fn parse_object(element: Rc<String>, object_type: &str) -> Result<FacilityObject, FloorParseError> {
    match object_type {
        "-compatible microchip" => Ok(FacilityObject::Microchip(element)),
        " generator"            => Ok(FacilityObject::Generator(element)),
        _                       => Err(FloorParseError::UnrecognizedObject),
    }
}

fn parse_floor_number(input: &str) -> Result<usize, FloorParseError> {
    match input {
        "first"  => Ok(1),
        "second" => Ok(2),
        "third"  => Ok(3),
        "fourth" => Ok(4),
        _        => Err(FloorParseError::UnrecognizedFloorNumber),
    }
}