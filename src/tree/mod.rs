//! Tree-based Genetic Programming implementation.
//!
//! This module provides:
//! - [`core`] - Core tree structures and individuals
//! - [`operators`] - Tree-specific evolutionary operators
//! - [`fitness`] - Fitness evaluation functions for trees

pub mod core;

pub mod operators;

pub mod fitness;

pub use self::core::tree::TreeGenotype;
pub use self::core::individual::TreeIndividual;
pub use self::operators::{
    init::*,
    mutation::*,
    crossover::*,
    select::*
};
pub use self::fitness::evaluate::*;
