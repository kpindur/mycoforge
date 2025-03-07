//! Operator sampling functionality for Genetic Programming.
//!
//! This module provides structures for weighted random sampling of operators based on their
//! arities and weights.

use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

/// Interface for sampling operators.
pub trait Sampler {
    /// Samples random operator.
    ///
    /// # Returns
    /// * `(String, usize)` - (operator name, arity) tuple
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

/// Sampler for operators with weights and arity constraints.
///
/// # Fields
/// * `operators: Vec<String>` - list of operator names
/// * `arity: Vec<usize>` - list of operator arities
/// * `weights: Vec<f64>` - list of sampling weights
#[derive(Clone)]
pub struct OperatorSampler {
    operators: Vec<String>,
    arity:     Vec<usize>,
    weights:   Vec<f64>,
}

impl OperatorSampler {
    pub fn new(operators: Vec<String>, arity: Vec<usize>, weights: Vec<f64>) -> Self {
        return Self { operators, arity, weights };
    }

    pub fn operators(&self) -> &Vec<String> { return &self.operators; }
    pub fn arities(&self) -> &Vec<usize> { return &self.arity; }
    pub fn weights(&self) -> &Vec<f64> { return &self.weights; }
    
    /// Updates sampling weights.
    ///
    /// # Panic
    /// * If new weights length doesn't match current weights
    pub fn update_weights(&mut self, weights: Vec<f64>) {
        assert_eq!(self.weights.len(), weights.len());
        self.weights = weights;
    }

    /// Creates new sampler with operators filtered by arity range.
    ///
    /// # Arguments
    /// * `min_arity: usize` - minimum allowed arity
    /// * `max_arity: usize` - maximum allowed arity
    ///
    /// # Returns
    /// * [`OperatorSampler`] - new sampler containing only operators with arities in range
    pub fn sampler_with_arity(&self, min_arity: usize, max_arity: usize) -> OperatorSampler {
        let is_valid = |arity| -> bool {
            return arity >= min_arity && arity <= max_arity;
        };
        let (mut filtered_operators, mut filtered_arity, mut filtered_weights) = (Vec::new(), Vec::new(), Vec::new());
        for (i, &arity) in self.arity.iter().enumerate() {
            if is_valid(arity) {
                filtered_operators.push(self.operators[i].clone());
                filtered_arity.push(self.arity[i]);
                filtered_weights.push(self.weights[i]);
            }
        }
        return Self { operators: filtered_operators, arity: filtered_arity, weights: filtered_weights };
    }

    /// Samples just the index of an operator rather than returning the operator itself
    pub fn sample_index<R: Rng>(&self, rng: &mut R) -> usize {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        return dist.sample(rng);
    }
}

impl Sampler for OperatorSampler {
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize) {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        let index: usize = dist.sample(rng);

        return (self.operators[index].clone(), self.arity[index]);
    }
}
