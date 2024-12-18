//! Evaluation functions for Genetic Programming
//!
//! This module provides evaluator for GP algorithms designed for assessing 
//! fitness of [`TreeGenotype`][`crate::tree::core::tree::TreeGenotype`]
//!
//! Currently implmeneted:
//! - Mean Squared Error (MSE)
//!
//! Also serves as a template for custom evaluation functions.
use std::collections::HashMap;

use crate::common::traits::Data;
use crate::{common::traits::Evaluator, tree::core::tree::TreeGenotype};
use crate::dataset::core::Dataset;

//Mean Squared Error (MSE) - most popular
//Root Mean Squared Error (RMSE)
//Mean Absolute Error (MAE)

/// Mean Squared Error (MSE) evaluator that computes fitness as average squared
/// difference between predicted and actual values.
///
/// # Examples
/// ```
/// use mycoforge::tree::fitness::evaluate::MeanSquared;
///
/// let evaluator = MeanSquared::default(); // Empty just for method encapsulation
/// ```
pub struct MeanSquared {}

impl MeanSquared {
    // Creates new MeanSquared evaluator.
    pub fn new() -> Self { return Self {}; }
}

impl Default for MeanSquared {
    fn default() -> Self { return Self {}; }
}

impl Evaluator<TreeGenotype> for MeanSquared {
    type D = Dataset;

    fn evaluate(&self, tree: &TreeGenotype, data: &Self::D, map: &HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)>) -> f64 {
        let mut stack: Vec<Vec<f64>> = Vec::new();

        for i in (0..tree.arena().len()).rev() {
            let node = &tree.arena()[i];

            if let Some((arity, op)) = map.get(node) {
                match arity {
                    0 => {
                        let operands = data.data_train().1.iter().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                        let result = op(&operands);
                        stack.push(result);
                    },
                    n => {
                        let mut operands = Vec::new();
                        for _ in 0..*n {
                            operands.push(stack.pop().unwrap());
                        }
                        let operands = operands.iter().rev().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                        let result = op(&operands);
                        stack.push(result);
                    },
                }
            }
        }
        let (no_dims, operands) = data.data_train();
        let truths = &operands[no_dims];
        let result = stack.pop().unwrap().iter()
            .zip(truths.iter())
            .map(|(t,y )| (t - y).powi(2))
            .sum::<f64>();
        return result;
    }

    fn memoized_evaluate(&self, 
            tree: &TreeGenotype, data: &Self::D, 
            map: &HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)>,
            cache: &HashMap<TreeGenotype, f64>
        ) -> f64 {
        if let Some(&value) = cache.get(tree) { return value; }

        return self.evaluate(tree, data, map);
    }
}
