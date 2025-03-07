//! Genetic Programming framework implemented in Rust.
//!
//! This crate provides tools for:
//! - Building and evaluating genetic programs (tree-based)
//! - Common evolutionary operators (mutation, crossover, selection)
//! - Dataset handling for supervised learning
//! - Standard optimizers and evaluation functions
//!
//! # Main Modules
//! - [`common`] - Core traits and types
//! - [`operators`] - Evolutionary operators and function sets
//! - [`dataset`] - Dataset handling utilities
//! - [`tree`] - Tree-based genetic Programming
//! - [`optimizers`] - Optimization algorithms

#![allow(clippy::needless_return)]

pub mod common;

pub mod operators;

pub mod dataset;

pub mod loggers;

//pub mod linear;

pub mod tree;

//pub mod graph;

//pub mod grammatical;

//pub mod utils;

//pub mod population;

pub mod optimizers;

pub mod prelude {
    pub mod tree_gp {
        pub use crate::common::traits::*;
        pub use crate::operators::builder::OperatorsBuilder;
        pub use crate::operators::functions::symbolic::*;
        pub use crate::tree::components::*;
    }
}
