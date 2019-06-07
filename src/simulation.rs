use crate::hotelling_agent::*;

use spaces::{Matrix};
use rsrl::geometry::{
	product::LinearSpace,
	continuous::Interval,
};
use voronoi::{voronoi, Point, make_polygons};
use crate::plotter::plot_points;

#[derive(Debug, Clone)]
pub struct Simulation {
	pub dim: Dimensions,
	pub round: u32,
	pub num_agents: u32,
	pub agents: Vec<HotellingAgent>,
	pub action_space: LinearSpace<Interval>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dimensions {
	OneD,
	TwoD,
}

impl Simulation {
	pub fn new(num_agents: u32, dim: Dimensions, agent_type: HotellingAgentType) -> Self {
		Simulation {
			dim: dim.clone(),
			round: 0,
			num_agents,
			agents: {
				match dim {
					Dimensions::OneD => (0..num_agents).map(|i| HotellingAgent::new(i, 1, agent_type)).collect(),
					Dimensions::TwoD => (0..num_agents).map(|i| HotellingAgent::new(i, 2, agent_type)).collect(),
				}
			},
			action_space: {
				match dim {
					Dimensions::OneD => LinearSpace::new((0..1).map(|_| Interval::bounded(0.0, 1.0)).collect()),
					Dimensions::TwoD => LinearSpace::new((0..2).map(|_| Interval::bounded(0.0, 1.0)).collect()),
				}
			}
		}
	}

	pub fn step(&mut self) -> Result<(), SynthesisError> {
		// Get positions of all agents
		let positions: Matrix<f64> = Self::get_matrix(self.agents.clone().into_iter()
			.map(|agent| agent.position.to_vec())
			.collect());
		plot_points(positions.clone());
		// Get new moves of all agents
		let _moves: Matrix<f64> = Self::get_matrix(self.agents.clone().into_iter()
			.map(|agent|
				agent.make_move(&positions, &self.action_space).unwrap().to_vec()
			).collect());
		// Create point representation of moves
		let points: Vec<Point> = {
			self.agents.clone().into_iter().map(|agent| { match self.dim {
				Dimensions::OneD => Point::new(agent.position[0], 0.0),
				Dimensions::TwoD => Point::new(agent.position[0], agent.position[1]),
			}}).collect()
		};
		// Find voronoi cells for each point
		let voronoi_diagram = voronoi(points, 1.0);
		let _vor_polys = make_polygons(&voronoi_diagram);
		println!("{:?}", _vor_polys);
		// Increment round
		self.round += 1;
		Ok(())
	}

	pub fn get_matrix(vec_of_vecs: Vec<Vec<f64>>) -> Matrix<f64> {
		// Matrix of `num_agents` rows and `dims` colums
		Matrix::from_shape_fn((vec_of_vecs.len(), vec_of_vecs[0].len()),
			|(i, j)| vec_of_vecs[i][j]
		)
	}
}

#[cfg(test)]
mod test {
	use crate::simulation::*;

	#[test]
	fn initialize_simulations() {
		let sim1 = Simulation::new(10, Dimensions::OneD, HotellingAgentType::Simple);
		let sim2 = Simulation::new(10, Dimensions::TwoD, HotellingAgentType::Simple);
		assert_eq!(sim1.round, 0);
		assert_eq!(sim2.round, 0);
		assert_eq!(sim1.dim, Dimensions::OneD);
		assert_eq!(sim2.dim, Dimensions::TwoD);
		assert_eq!(sim1.num_agents, 10);
		assert_eq!(sim2.num_agents, 10);
	}

	#[test]
	fn step_simulation() {
		let mut sim = Simulation::new(10, Dimensions::OneD, HotellingAgentType::Random);
		let result = sim.step();
		assert_eq!(result, Ok(()));
	}
}