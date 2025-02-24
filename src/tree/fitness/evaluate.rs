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
use crate::common::types::VectorFunction;
use crate::{common::traits::Evaluator, tree::core::tree::TreeGenotype};
use crate::dataset::core::Dataset;


fn common_evaluate(
    stack: &mut  Vec<Vec<f64>>, tree: &TreeGenotype,
    dataset: &[Vec<f64>], map: &HashMap<String, (usize, VectorFunction)>
) {
    for i in (0..tree.arena().len()).rev() {
        let node = &tree.arena()[i];

        if let Some((arity, op)) = map.get(node) {
            match arity {
                0 => {
                    let operands = dataset.iter().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                    let result = op(&operands);
                    stack.push(result);
                },
                n => {
                    let mut operands = Vec::new();
                    for _ in 0..*n {
                        operands.push(stack.pop().unwrap());
                    }
                    let operands = operands.iter().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                    let result = op(&operands);
                    stack.push(result);
                },
            }
        }
    }
}

//Sum of Square Errors (SSE)
//Mean Squared Error (MSE) - most popular
//Root Mean Squared Error (RMSE)
//Mean Absolute Error (MAE)

/// Sum of Square Errors (SSE) evaluator that computes fitness as the sum of squared
/// difference between predicted and actual values.
///
/// # Examples
/// ```
/// use mycoforge::tree::fitness::evaluate::SSE;
///
/// let evaluator = SSE::default(); // Empty just for method encapsulation
/// ```
pub struct SSE {}

impl SSE {
    pub fn new() -> Self { return Self {}; }
}

impl Default for SSE {
    fn default() -> Self { return Self::new(); }
}

impl Evaluator<TreeGenotype> for SSE {
    type D = Dataset;

    fn evaluate(&self, 
            tree: &TreeGenotype, dataset: &Self::D, 
            map: &HashMap<String, (usize, VectorFunction)>
        ) -> f64 {
        let mut stack: Vec<Vec<f64>> = Vec::new();
        let (features, target) = dataset.data();

        common_evaluate(&mut stack, tree, features, map);

        let predictions = stack.pop().unwrap();
        let result = predictions.iter()
            .zip(target.iter())
            .map(|(t,y )| {
                let diff = t - y;
                let sq = diff.powi(2);
                return sq;
            }).sum::<f64>();
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

/// Mean Squared Error (MSE) evaluator that computes fitness as average squared
/// difference between predicted and actual values.
///
/// # Examples
/// ```
/// use mycoforge::tree::fitness::evaluate::MSE;
///
/// let evaluator = MSE::default(); // Empty just for method encapsulation
/// ```
pub struct MSE {}

impl MSE {
    // Creates new MeanSquared evaluator.
    pub fn new() -> Self { return Self {}; }
}

impl Default for MSE {
    fn default() -> Self { return Self {}; }
}

impl Evaluator<TreeGenotype> for MSE {
    type D = Dataset;

    fn evaluate(&self, 
            tree: &TreeGenotype, dataset: &Self::D, 
            map: &HashMap<String, (usize, VectorFunction)>
        ) -> f64 {
        let mut stack: Vec<Vec<f64>> = Vec::new();
        let (features, target) = dataset.data();

        common_evaluate(&mut stack, tree, features, map);

        let predictions = stack.pop().unwrap();
        let result = predictions.iter()
            .zip(target.iter())
            .map(|(t,y )| {
                let diff = t - y;
                let sq = diff.powi(2);
                return sq;
            }).sum::<f64>();
        return result / (target.len() as f64);
    }

    fn memoized_evaluate(&self, 
            tree: &TreeGenotype, data: &Self::D, 
            map: &HashMap<String, (usize, VectorFunction)>,
            cache: &HashMap<TreeGenotype, f64>
        ) -> f64 {
        if let Some(&value) = cache.get(tree) { return value; }

        return self.evaluate(tree, data, map);
    }
}

/// Root Mean Square Error (RMSE) evaluator that computes fitness as square root of 
/// average squared difference between predicted and actual values.
///
/// # Examples
/// ```
/// use mycoforge::tree::fitness::evaluate::RMSE;
///
/// let evaluator = RMSE::default(); // Empty just for method encapsulation
/// ```
pub struct RMSE {}

impl RMSE {
    // Creates new MeanSquared evaluator.
    pub fn new() -> Self { return Self {}; }
}

impl Default for RMSE {
    fn default() -> Self { return Self {}; }
}

impl Evaluator<TreeGenotype> for RMSE {
    type D = Dataset;

    fn evaluate(&self, 
        tree: &TreeGenotype, dataset: &Self::D, 
        map: &HashMap<String, (usize, VectorFunction)>
    ) -> f64 {
        let mut stack: Vec<Vec<f64>> = Vec::new();
        let (features, target) = dataset.data();

        common_evaluate(&mut stack, tree, features, map);

        let predictions = stack.pop().unwrap();
        let result = predictions.iter()
            .zip(target.iter())
            .map(|(t,y )| {
                let diff = t - y;
                let sq = diff.powi(2);
                return sq;
            }).sum::<f64>();
        return (result / (target.len() as f64)).sqrt();
    }

    fn memoized_evaluate(&self, 
            tree: &TreeGenotype, data: &Self::D, 
            map: &HashMap<String, (usize, VectorFunction)>,
            cache: &HashMap<TreeGenotype, f64>
        ) -> f64 {
        if let Some(&value) = cache.get(tree) { return value; }

        return self.evaluate(tree, data, map);
    }
}
