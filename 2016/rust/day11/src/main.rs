#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate typed_arena;

mod facilitystate;
mod floor;
mod helpers;
mod solver;

use solver::Solver;

fn main() {
    let path = "example1.txt";
    let mut solver = Solver::parse(path).unwrap();
    solver.solve();
}
