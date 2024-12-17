//! Tree crossover operators for Genetic Programming
//!
//! This module provides crossover operators for tree-based GP designed for manipulating
//! [`TreeGenotype`][`crate::tree::core::tree::TreeGenotype`] structure. 
//! Also serves as a template for custom crossover operators.

use rand::Rng;

use crate::common::traits::Crossoverer;
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::OperatorSampler;
use crate::tree::operators::errors::CrossoverError;

use log::{error, debug, info};

/// Traditional subtree crossover operator that swaps randomly selected subtrees between parents.
///
/// # Fields
/// * `probability: f64` - Crossover probability (0.0 to 1.0)
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::crossover::SubtreeCrossover;
///
/// let crossover = SubtreeCrossover::new(0.9);
/// ```
pub struct SubtreeCrossover {
    probability: f64,
}

impl Default for SubtreeCrossover {
    fn default() -> Self {
        debug!("Creating default SubtreeCrossover with probability {}", 0.7);
        return Self::new(0.7).expect("Failed to create default SubtreeCrossover!")
    }
}

impl SubtreeCrossover {
    /// Creates new SubtreeCrossover operator.
    ///
    /// # Arguments
    /// * `probability: f64` - crossover probability (0.0 to 1.0)
    ///
    /// # Returns
    /// * `Result<Self, CrossoverError>` - instance of Self or an
    /// [`Error`][`crate::tree::operators::errors::CrossoverError`]
    pub fn new(probability: f64) -> Result<Self, CrossoverError> {
        if !(0.0..=1.0).contains(&probability) { 
            error!("Attempted to crate SubtreeCrossover with invalid probability: {}", probability);
            return Err(CrossoverError::InvalidProbability(probability));
        }
        return Ok(Self { probability });
    }
    
    /// Swaps subtrees between parents at specified crossover points.
    ///
    /// # Arguments
    /// * `parents: (&TreeGenotype, &TreeGenotype)` - parent
    /// [`trees`][`crate::tree::core::tree::TreeGenotype`] for crossover
    /// * `crossover_points: (usize, usize)` - indices where subtree swap occurs
    ///
    /// # Returns
    /// * `Vec<Vec<String>>` - arenas of two offspring after subtree swap
    fn swap(parents: (&TreeGenotype, &TreeGenotype), crossover_points: (usize, usize)) 
        -> Vec<Vec<String>> {
        let (parent1, parent2) = parents;
        let (xo_point1, xo_point2) = crossover_points;

        let sub_end1 = parent1.subtree(xo_point1);
        let sub_end2 = parent2.subtree(xo_point2);

        let subtree1 = &parent1.arena()[xo_point1..=sub_end1];
        let subtree2 = &parent2.arena()[xo_point2..=sub_end2];

        let mut tree1 = parent1.arena()[..xo_point1].to_vec();
        tree1.extend_from_slice(subtree2);
        tree1.extend_from_slice(&parent1.arena()[sub_end1+1..]);

        let mut tree2 = parent2.arena()[..xo_point2].to_vec();
        tree2.extend_from_slice(subtree1);
        tree2.extend_from_slice(&parent2.arena()[sub_end2+1..]);

        return vec![tree1, tree2];
    }

}

impl Crossoverer<TreeGenotype> for SubtreeCrossover {
    fn variate<R: Rng>(&self, rng: &mut R, parent1: &TreeGenotype, parent2: &TreeGenotype, sampler: &OperatorSampler) -> Vec<TreeGenotype> {
        if rng.gen::<f64>() > self.probability { return [parent1.clone(), parent2.clone()].to_vec(); }

        let crossover_points: (usize, usize) = (rng.gen_range(0..parent1.arena().len()), rng.gen_range(0..parent2.arena().len()));
        let trees = Self::swap( (parent1, parent2), crossover_points);
        // Change arena.clone() to &arena in the future
        let mut mutants = Vec::new();
        for tree in trees {
            let mut child = TreeGenotype::with_arena(tree.clone());
            *child.children_mut() = child.construct_children(sampler);
            mutants.push(child);
        }

        return mutants;
    }
}
