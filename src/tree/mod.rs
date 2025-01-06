//! Tree-based Genetic Programming implementation.
//!
//! This module provides:
//! - [`core`] - Core tree structures and individuals
//! - [`operators`] - Tree-specific evolutionary operators
//! - [`fitness`] - Fitness evaluation functions for trees

pub mod core;

pub mod operators;

pub mod fitness;

pub mod components {
    pub use super::core::tree::TreeGenotype;
    pub use super::core::individual::TreeIndividual;
    pub use super::operators::{
        init::*,
        mutation::*,
        crossover::*,
        select::*
    };
    pub use super::fitness::evaluate::*;
}
