use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::hash_map::Entry;

pub fn dedup<T: Eq + Hash>(map: &mut HashMap<Rc<T>, ()>, value: T) -> Rc<T> {
    match map.entry(Rc::new(value)) {
        Entry::Occupied(entry) => {
            entry.key().clone()
        },
        Entry::Vacant(entry)   => {
            let output = entry.key().clone();
            entry.insert(());
            output
        }
    }
}
