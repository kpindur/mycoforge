//! Error types used across operator implementations.

use std::fmt;
use std::error::Error;

/// Errors that can occur during mutation operations.
///
/// # Variants
/// * `InvalidProbability(f64)` - mutation probability outside [0.0, 1.0] range
/// * `InvalidMutationRate(f64)` - mutation rate outside [0.0, 1.0] range
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

/// Errors that can occur during crossover operations.
/// 
/// # Variants
/// * `InvalidProbability(f64)` - crossover probability outside [0.0, 1.0] range
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

/// Errors that can occur during selection operations.
///
/// # Variants
/// * `InvalidTournamentSize((usize, usize))` - tournament size exceeds population size
/// * `InvalidFitnessComparison((f64, f64))` - failed to compare fitness values
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
