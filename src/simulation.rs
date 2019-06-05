use crate::hotelling_agent::*;
use spaces::{Vector, Matrix};
use rsrl::geometry::{
    product::LinearSpace,
    continuous::Interval,
};

pub struct Simulation {
    round: u32,
    num_agents: u32,
    agents: Vec<HotellingAgent>,
    action_space: LinearSpace<Interval>,
}

impl Simulation {
    pub fn new(num_agents: u32, dim: u32) -> Self {
        Simulation {
            round: 0,
            num_agents,
            agents: (0..num_agents).map(|i|
                HotellingAgent::new(i, dim, HotellingAgentType::Simple, None)
            ).collect(),
            action_space: LinearSpace::new(
                (0..dim).map(|_| Interval::bounded(0.0, 1.0)).collect()
            ),
        }
    }

    pub fn step(self) -> Result<(), SynthesisError> {
        let positions: Matrix<f64> = Self::get_matrix(self.agents.clone().into_iter()
            .map(|agent| agent.position.to_vec())
            .collect());
        let moves: Matrix<f64> = Self::get_matrix(self.agents.clone().into_iter()
            .map(|agent|
                agent.make_move(&positions, &self.action_space).unwrap().to_vec()
            ).collect());
        let rewards: Matrix<f64> = Self::calculate_rewards(&moves);
        Ok(())
    }

    pub fn get_matrix(vec_of_vecs: Vec<Vec<f64>>) -> Matrix<f64> {
        Matrix::from_shape_fn((vec_of_vecs.len(), vec_of_vecs.len()),
            |(i, j)| vec_of_vecs[i][j]
        )
    }
}