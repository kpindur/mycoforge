use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum MutationError {
    InvalidProbability(f64),
    InvalidMutationRate(f64)
}

impl Error for MutationError {}

impl fmt::Display for MutationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MutationError::InvalidProbability(probability) 
                => write!(f, "Invalid mutation probability: {}", probability),
            MutationError::InvalidMutationRate(mutation_rate)
                => write!(f, "Invalid mutation rate: {}", mutation_rate),
        }
    }
}
