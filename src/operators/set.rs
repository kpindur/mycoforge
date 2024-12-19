//! Operator sets and builder for managing GP operators.
//!
//! This module provides structures for:
//! - Managing sets of operators with their arities and weights
//! - Building operator sets with validation
//! - Sampling operators based on weights

use rand::Rng;
use std::collections::HashMap;

use crate::operators::sampler::{OperatorSampler, Sampler};
use crate::common::types::VectorFunction;

/// Interface for operator sets with sampling capability
pub trait OperatorSet {
    /// Returns operator by name.
    fn get_operator(&self, name: &str) -> Option<&Functor>;
    /// Samples random operator based on weights.
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

/// Container for operators with sampling functionality.
///
/// # Fields
/// * `operators: HashMap<String, Functor>` - Map of operator names to functors
/// * `sampler: OperatorSampler` - Sampler for random operator selection
pub struct Operators {
    operators: HashMap<String, Functor>,
    sampler: OperatorSampler,
}

impl Operators {
    pub fn new(operators: HashMap<String, Functor>, sampler: OperatorSampler) -> Self { 
        return Self { operators, sampler }; 
    }
    
    pub fn operators(&self) -> &HashMap<String, Functor> { return &self.operators; }
    pub fn sampler(&self) -> &OperatorSampler { return &self.sampler; }

    pub fn operators_mut(&mut self) -> &mut HashMap<String, Functor> { return &mut self.operators; }
    pub fn sampler_mut(&mut self) -> &mut OperatorSampler { return &mut self.sampler; }
    
    /// Creates map of operators with their arities and functions. Required for tree evaluations.
    pub fn create_map(&self) -> HashMap<String, (usize, VectorFunction)> {
        let mut map = HashMap::new();
        for (key, value) in &self.operators {
            map.insert(key.clone(), (value.arity(), *value.func()));
        }
        return map;
    }
}

impl OperatorSet for Operators {
    fn get_operator(&self, name: &str) -> Option<&Functor> { return self.operators.get(name); }
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize) { return self.sampler.sample(rng); }
}

/// Wrapper for operator function with metadata.
///
/// # Fields
/// * `func: VectorFunction` - [`function`][`crate::common::types::VectorFunction`] implementing
/// the operator
/// * `arity: usize` - number of arguments operator takes
/// * `weight: f64` - sampling weight for operator
#[derive(Clone)]
pub struct Functor {
    func: VectorFunction,
    arity: usize,
    weight: f64
}

impl Functor {
    pub fn new(func: VectorFunction, arity: usize, weight: f64) -> Self { return Self { func, arity, weight }; }

    pub fn arity(&self) -> usize { return self.arity; }
    pub fn weight(&self) -> f64 { return self.weight; }
    pub fn func(&self) -> &VectorFunction { return &self.func; }
}

/// Errors that can occur during operator set building.
///
/// # Variants
/// * `IncorrectWeight` - weight not in (0, 1] range
/// * `KeyExists` - operator with given name already exists 
/// * `OperatorsIsEmpty` - no operators added to builder
/// * `WrongWeightSum` - weights don't sum to 1.0
#[derive(Debug)]
pub enum BuilderError {
    IncorrectWeight,
    KeyExists,
    OperatorsIsEmpty,
    WrongWeightSum
}

impl std::error::Error for BuilderError {}
impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectWeight => write!(f, "IncorrectWeight"),
            Self::KeyExists => write!(f, "KeyExists"),
            Self::OperatorsIsEmpty => write!(f, "OperatorsIsEmpty"),
            Self::WrongWeightSum => write!(f, "WrongWeightSum"),
        }
    }
}

/// Builder for creating validated operator sets.
///
/// # Fields
/// * `operators: HashMap<String, Functor>` - map of operators to be validated
/// * `weights_sum: f64` - running sum of operator weights
pub struct OperatorsBuilder {
    operators: HashMap<String, Functor>,
    weights_sum: f64,
}

impl OperatorsBuilder {
    pub fn new(operators: HashMap<String, Functor>, weights_sum: f64) -> Self {
        return Self { operators, weights_sum };
    }
    
    /// Adds new operator to the set.
    ///
    /// # arguments
    /// * `name: &str` - operator name
    /// * `func: VectorFunction` - operator function
    /// * `arity: usize` - number of arguments
    /// * `weight: f64` - sampling weight
    ///
    /// # Returns
    /// * `Result<Self, BuilderError>` - Update builder or [`error`][`BuilderError`]
    pub fn add_operator(mut self, name: &str, func: VectorFunction, arity: usize, weight: f64) 
        -> Result<Self, BuilderError> {
            if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
            if self.operators.contains_key(name) { return Err(BuilderError::KeyExists); }
            
            self.operators.insert(name.to_string(), Functor::new(func, arity, weight));
            self.weights_sum += weight;

            return Ok(self);
    }
    
    /// Builds final operator set with validation.
    ///
    /// # Returns
    /// * `Result<Operators, BuilderError>` - Valid [`operator set`][Operators] or
    /// [`error`][`BuilderError`]
    pub fn build(self) -> Result<Operators, BuilderError> {
        if self.operators.is_empty() { return Err(BuilderError::OperatorsIsEmpty); }
        if (self.weights_sum - 1.0).abs() > 1e-10 { return Err(BuilderError::WrongWeightSum); }
        
        let capacity = self.operators.len();
        let (mut ops, mut arity, mut weights) = 
            (Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity));

        for (name, func) in &self.operators {
            ops.push(name.clone());
            arity.push(func.arity());
            weights.push(func.weight());
        }

        return Ok(Operators::new(self.operators, OperatorSampler::new(ops, arity, weights)));
    }
}

impl Default for OperatorsBuilder {
    fn default() -> Self { return Self { operators: HashMap::new(), weights_sum: 0.0 }; }
}
