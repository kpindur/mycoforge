//! Evolutionary operators for tree-based Genetic Programming.
//!
//! This module provides:
//! - [`init`] - Tree initialization methods
//! - [`errors`] - Error types for operator operations
//! - [`mutation`] - Tree mutation operators
//! - [`crossover`] - Tree crossover operators
//! - [`select`] - Selection operators

pub mod init;

pub mod errors;
pub mod mutation;
pub mod crossover;
pub mod select;
