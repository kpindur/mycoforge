//! Tree mutation operators for Genetic Programming (GP)
//!
//! This module provides mutation operators for tree-based GP designed for manipulating
//! `TreeGenotype` structure. Also serves as a template for custom mutation operators.

use rand::Rng;

use crate::common::traits::{Initializer, Mutator};
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::{OperatorSampler, Sampler};

use super::init::Grow;

use super::errors::MutationError;
use log::{info, error, debug};

/// Replaces a subtree at the specified mutation point in the individual with a new subtree
fn substitute(individual: &TreeGenotype, subtree: &TreeGenotype, mutation_point: usize) 
    -> Vec<String> {
    let mutation_end: usize = individual.subtree(mutation_point);

    let mut new_arena = individual.arena()[0..mutation_point].to_vec();
    new_arena.extend(subtree.arena().iter().cloned());
    new_arena.extend(individual.arena()[mutation_end+1..].iter().cloned());

    return new_arena;
}

/// Traditional subtree mutation operator that replaces a randomly selected subtree with a new one
/// generated using the Grow initialization method.
///
/// # Fields:
/// * `probability: f64` - Mutation probabilkity (0.0 to 1.0)
/// * `depth_limits: (usize, usize)` - Min and max depth for new subtrees (inclusive) 
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::mutation::SubtreeMutation;
///
/// let mutation = SubtreeMutation::default();
/// ```
pub struct SubtreeMutation {
    probability: f64,
    depth_limits: (usize, usize)
}

impl Default for SubtreeMutation {
    fn default() -> Self {
        debug!("Creating default SubtreeMutation with probability {} and depth limits ({}, {})", 0.1, 1, 2);
        return Self::new(0.1, (1, 2)).expect("Failed to create default SubtreeMutation!");
    }
}

impl SubtreeMutation {
    pub fn new(probability: f64, depth_limits: (usize, usize)) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to crate SubtreeMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        info!("Created SubtreeMutation operator with probability {} and depth limits ({}, {})", probability, depth_limits.0, depth_limits.1);
        return Ok(Self { probability, depth_limits });
    }
}

impl Mutator<TreeGenotype> for SubtreeMutation {
    /// Performs subtree mutation on the given individual.
    ///
    /// Return the original individual unchanged if random number exceeds probability.
    /// Otherwise return mutated individual.
    fn variate<R: Rng>(&self, rng: &mut R, individual: &TreeGenotype, sampler: &OperatorSampler) -> TreeGenotype {
        if rng.gen::<f64>() > self.probability { 
            debug!("Skipping mutation..");
            return individual.clone(); 
        }
        
        let mutation_point: usize = rng.gen_range(0..individual.arena().len());
        
        let init_scheme = Grow::new(self.depth_limits.0, self.depth_limits.1);
        let subtree = init_scheme.initialize(rng, sampler);
        debug!("Generated subtree of size {} at point {}", subtree.arena().len(), mutation_point);
        
        let arena = substitute(individual, &subtree, mutation_point);
        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);
        
        debug!("Completed mutation: original size {} -> mutant size {}", individual.arena().len(), tree.arena().len());
        return tree.clone();
    }
}

/// Advanced mutation operator that generates replacement subtree with sizes proportional to the
/// original tree or subtree size.
///
/// # Fields:
/// * `probability: f64` - Mutation probability (0.0 to 1.0)
/// * `dynamic_limit: bool` - when true, uses subtree size instead of full tree size for depth
///                         calculations
/// # Size Calculation:
/// * Minimum depth = log2(tree_size / 2)
/// * Maximum depth = log2(tree_size * 1.5)
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::mutation::SizeFairMutation;
///
/// let mutation = SizeFairMutation::default();
/// ```
pub struct SizeFairMutation {
    probability: f64,
    dynamic_limit: bool
}

impl Default for SizeFairMutation {
    fn default() -> Self {
        debug!("Creating default SizeFairMutation with probability {} and dynamic limit {}", 0.1, false);
        return Self::new(0.1, false).expect("Failed to create default SizeFairMutation!");
    }
}

impl SizeFairMutation {
    pub fn new(probability: f64, dynamic_limit: bool) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to create SizeFairMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        info!("Created SizeFairMutation operator with probability {} and dynamic limit {}", probability, dynamic_limit);
        return Ok(Self { probability, dynamic_limit });
    }
    /// Calculates minimum and maximum depths based on tree size.
    ///
    /// When `dynamic_limit` is true, uses the size of the subtree at `mutation_point`.
    /// Otherwise, uses the size of the entire tree.
    fn calculate_depths(&self, tree: &TreeGenotype, mutation_point: usize) -> (usize, usize) {
        let tree_size = if self.dynamic_limit {
            let subtree_end = tree.subtree(mutation_point);
            (subtree_end - mutation_point) as f64
        } else {
            tree.arena().len() as f64
        };

        let depth_min = (tree_size / 2.0).floor().log2() as usize;
        let depth_max = (tree_size * 1.5).ceil().log2() as usize;

        return (depth_min, depth_max);
    }
}

impl Mutator<TreeGenotype> for SizeFairMutation {
    /// Performs size-fair mutation on the given individual.
    ///
    /// Returns the original individual unchanged if random number exceeds probability.
    /// Otherwise returns mutated individual.
    fn variate<R: Rng>(&self, rng: &mut R, individual: &TreeGenotype, sampler: &OperatorSampler) -> TreeGenotype {
        if rng.gen::<f64>() > self.probability {
            debug!("Skipping mutation");
            return individual.clone();
        }

        let mutation_point = rng.gen_range(0..individual.arena().len());
        let depth_limits = self.calculate_depths(individual, mutation_point);

        let init_scheme = Grow::new(depth_limits.0, depth_limits.1);
        let subtree = init_scheme.initialize(rng, sampler);

        let arena = substitute(individual, &subtree, mutation_point);
        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);

        debug!("Completed mutation: original size {} -> mutant size {}", individual.arena().len(), tree.arena().len());
        return tree.clone();
    }
}

/// Point mutation operator (aka node replacement mutation) that replaces a randomly selected node
/// with another node of the same arity.
///
/// # Fields:
/// * `probability: f64` - Mutation probability (0.0 to 1.0)
///
/// # Examples:
/// ```
/// use mycoforge::tree::operators::mutation::PointMutation;
///
/// let mutation = PointMutation::default();
/// ```
pub struct PointMutation {
    probability: f64
}

impl Default for PointMutation {
    fn default() -> Self {
        debug!("Creating default PointMutation with probability {}", 0.1);
        return Self::new(0.1).expect("Failed to create default PointMutation!");
    }
}

impl PointMutation {
    pub fn new(probability: f64) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to create PointMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        info!("Created PointMutation operator with probability {}", probability);
        return Ok(Self { probability });
    }
}

impl Mutator<TreeGenotype> for PointMutation {
    /// Performs point mutation on the given individual.
    ///
    /// Returns the original individual unchanged if random number exceeds probability.
    /// Otherwise returns mutated individual.
    fn variate<R: Rng>(&self, rng: &mut R, individual: &TreeGenotype, sampler: &OperatorSampler) -> TreeGenotype {
        if rng.gen::<f64>() > self.probability { 
            debug!("Skipping mutation..");
            return individual.clone(); 
        }
        
        let mutation_point: usize = rng.gen_range(0..individual.arena().len());
        let index = sampler.operators().iter()
            .position(|s| *s == individual.arena()[mutation_point])
            .expect("Failed to find operator in given sampler!");
        let arity = sampler.arities()[index];
        let limited_sampler = sampler.sampler_with_arity(arity, arity);
        let new_node = limited_sampler.sample(rng);
        assert_eq!(new_node.1, arity,
            "Generated new node with different arity! Expected {}, found {}", arity, new_node.1
        );
        debug!("Generated new node {} with arity {}", new_node.0, new_node.1);
        let mut arena = individual.arena().clone();
        arena[mutation_point] = new_node.0;
        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);
        
        debug!("Completed mutation: original size {} -> mutant size {}", individual.arena().len(), tree.arena().len());
        return tree.clone();
    }
}
