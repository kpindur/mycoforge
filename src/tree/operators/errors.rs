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

#[derive(Debug)]
pub enum CrossoverError {
    InvalidProbability(f64)
}

impl Error for CrossoverError {}

impl fmt::Display for CrossoverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossoverError::InvalidProbability(probability)
                => write!(f, "Invalid crossover probability: {}", probability),
        }
    }
}

#[derive(Debug)]
pub enum SelectionError {
    InvalidTournamentSize((usize, usize)),
    InvalidFitnessComparison((f64, f64))
}

impl Error for SelectionError {}

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionError::InvalidTournamentSize((tournament_size, population_size))
                => write!(f, "Tournament size {} exceeds population size {}!", tournament_size, population_size),
            SelectionError::InvalidFitnessComparison((a, b))
                => write!(f, "Invalid fitness comparison: {} ? {}", a, b)
        }
    }
}
