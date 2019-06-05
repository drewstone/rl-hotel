extern crate spaces;
extern crate arrayvec;

mod simulation;
mod hotelling_agent;

use simulation::Simulation;

fn main() {
    let sim = Simulation::new(10, 10);
}
