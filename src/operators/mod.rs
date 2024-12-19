//! Common operators and operator sets for Genetic Programming.
//!
//! This module provides:
//! - [`sampler`] - Core functionality for weighted random sampling of operators with arity
//! constraints
//! - [`functions`] - Common vectorized functions for symbolic regression (arithmetic,
//! trigonometric, etc.)
//! - [`set`] - Management of operator sets including builder batter for creating valid sets and
//! sampling functionality.

pub mod functions;

pub mod set;

pub mod sampler;
