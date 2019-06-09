extern crate ndarray;
extern crate spaces;
extern crate arrayvec;
extern crate geo;
extern crate voronoi;

mod simulation;
mod hotelling_agent;
mod plotter;

use simulation::Simulation;
use hotelling_agent::HotellingAgentType;
use simulation::Dimensions;
use plotter::plot_points;

fn main() {
	let mut sim = Simulation::new(10, Dimensions::TwoD, HotellingAgentType::Random);
	for _i in 0..1 {
		match sim.step() {
			Ok(pos) => {
				plot_points(pos);
			},
			Err(err) => panic!("{:?}", err),
		}
	}
}
