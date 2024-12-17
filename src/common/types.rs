//! Common type definitions used across the codebase.

/// Function type for vectorized operations on data.
/// 
/// # Arguments
/// * `&[&[f64]]` - slice of feature vectors
/// 
/// # Returns
/// * `Vec<f64>` - result of vectorized operation
pub type VectorFunction = fn(&[&[f64]]) -> Vec<f64>;
