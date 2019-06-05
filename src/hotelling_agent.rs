use rand::prelude::*;
use spaces::{Vector, Matrix};
use rsrl::geometry::{
    product::LinearSpace,
    continuous::Interval,
};

#[derive(Debug, Copy, Clone)]
pub enum HotellingAgentType {
	Simple,
	Random,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum SynthesisError {
    UnexpectedNoneType,
    InvalidAgentType,
}

#[derive(Debug, Clone)]
pub struct HotellingAgent {
    pub id: u32,
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
}

impl HotellingAgent {
    pub fn empty() -> Self {
        HotellingAgent {
            id: 0,
            position: Vector::zeros(0),
            velocity: Vector::zeros(0),
        }
    }

    pub fn new(id: u32, dim: u32, agent_type: HotellingAgentType, rng: Option<ThreadRng>) -> Self {
    	let (position, velocity) = match Self::setup_type(dim, agent_type, rng) {
            Ok((pos, vel)) => (pos, vel),
            Err(_) => (Self::empty().position, Self::empty().velocity),
        };

    	HotellingAgent { id, position, velocity }
    }

    pub fn setup_type(dim: u32, agent_type: HotellingAgentType, rng: Option<ThreadRng>)
        -> Result<(Vector, Vector), SynthesisError> {
    	match agent_type {
    		HotellingAgentType::Simple => Ok((
                Vector::from_vec((0..dim).map(|_| 1.0).collect()),
                Vector::from_vec((0..dim).map(|_| 1.0).collect())
            )),
    		HotellingAgentType::Random => {
                let mut r = rng.ok_or(SynthesisError::UnexpectedNoneType)?;
                Ok((
                    Vector::from_vec((0..dim).map(|_| { let x: f64 = r.gen(); x }).collect()),
                    Vector::from_vec((0..dim).map(|_| { let x: f64 = r.gen(); x }).collect())
                ))
            },
            HotellingAgentType::Unknown => Err(SynthesisError::InvalidAgentType)
    	}
    }

    pub fn make_move(self, agent_states: &Matrix, action_space: &LinearSpace<Interval>) -> Result<Vector, SynthesisError> {
        Ok(Vector::from_vec(vec![]))
    }
    pub fn process_reward() -> Result<(), SynthesisError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use spaces::{Vector};
	#[test]
	fn create_agents() {
        let agent_types: Vec<HotellingAgentType> = vec![
            HotellingAgentType::Simple,
            HotellingAgentType::Random,
            HotellingAgentType::Unknown
        ];

        for i in 0..agent_types.len() {
            match agent_types[i] {
                HotellingAgentType::Simple => {
                    let agent = HotellingAgent::new(0,0, HotellingAgentType::Simple, None);
                    assert_eq!(agent.position, Vector::from_vec(vec![]));
                    assert_eq!(agent.velocity, Vector::from_vec(vec![]));
                },
                HotellingAgentType::Random => {
                    let rng = rand::thread_rng();
                    let agent = HotellingAgent::new(0, 0, HotellingAgentType::Simple, Some(rng));
                    assert_eq!(agent.position, Vector::from_vec(vec![]));
                    assert_eq!(agent.velocity, Vector::from_vec(vec![]));
                },
                HotellingAgentType::Unknown => {
                    let agent = HotellingAgent::empty();
                    assert_eq!(agent.position, Vector::zeros(0));
                    assert_eq!(agent.velocity, Vector::zeros(0));
                },
            }
        }
	}

	#[test]
	fn new_agent_dim_1() {
        let agent_types: Vec<HotellingAgentType> = vec![
            HotellingAgentType::Simple,
            HotellingAgentType::Random,
        ];

		let dim = 1;
        for i in 0..agent_types.len() {
            match agent_types[i] {
                HotellingAgentType::Simple => {
                    let agent = HotellingAgent::new(0, dim, HotellingAgentType::Simple, None);
                    assert_eq!(agent.position.len(), 1);
                    assert_eq!(agent.velocity.len(), 1);
                },
                HotellingAgentType::Random => {
                    let rng = rand::thread_rng();
                    let agent = HotellingAgent::new(0, dim, HotellingAgentType::Simple, Some(rng));
                    assert_eq!(agent.position.len(), 1);
                    assert_eq!(agent.velocity.len(), 1);
                },
                _ => {},
            }
        }
	}
}