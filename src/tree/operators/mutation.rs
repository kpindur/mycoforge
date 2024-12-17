//! Tree mutation operators for Genetic Programming
//!
//! This module provides mutation operators for tree-based GP designed for manipulating
//! [`TreeGenotype`][`crate::tree::core::tree::TreeGenotype`] structure. Also serves as a template for custom mutation operators.

use rand::Rng;

use crate::common::traits::{Initializer, Mutator};
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::{OperatorSampler, Sampler};

use super::init::Grow;

use super::errors::MutationError;
use log::{info, error, debug};

/// Substitutes a subtree at the given mutation point with a new subtree.
///
/// # Arguments
/// * `individual: &TreeGenotype` - original [`TreeGenotype`][`crate::tree::core::tree::TreeGenotype`]
/// * `subtree: &TreeGenotype` - new subtree to insert
/// * `mutation_point: usize` - index where substitution occurs
///
/// # Returns
/// * `Vec<String>` - new tree arena after substitution
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
    /// Creates new SubtreeMutation operator.
    ///
    /// # Arguments
    /// * `probability: f64` - mutation probability (0.0 to 1.0)
    /// * `depth_limits: (usize, usize)` - min and max depth for new subtrees
    ///
    /// # Returns
    /// * `Result<Self, MutationError>` - new operator or error if probability invalid
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
    /// Creates new SizeFairMutation operator.
    ///
    /// # Arguments
    /// * `probability: f64` - mutation probability (0.0 to 1.0)
    /// * `dynamic_limit: bool` - use subtree size instead of full tree size
    ///
    /// # Returns
    /// * `Result<Self, MutationError>` - new operator or error if probability invalid
    pub fn new(probability: f64, dynamic_limit: bool) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to create SizeFairMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        info!("Created SizeFairMutation operator with probability {} and dynamic limit {}", probability, dynamic_limit);
        return Ok(Self { probability, dynamic_limit });
    }

    /// Calculates depth limits based on tree or subtree size.
    ///
    /// # Arguments
    /// * `tree: &TreeGenotype` - tree to calculate limits for
    /// * `mutation_point: usize` - point of mutation (used with dynamic_limit)
    ///
    /// # Returns
    /// * `(usize, usize)` - calculated (min, max) depth limits
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
    /// Creates new PointMutation operator.
    ///
    /// # Arguments
    /// * `probability: f64` - mutation probability (0.0 to 1.0)
    ///
    /// # Returns
    /// * `Result<Self, MutationError>` - new operator or error if probability invalid
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

/// Mutation operator that modifies constant values in the tree by a random factor.
///
/// # Fields
/// * `probability: f64` - Mutation probability (0.0 to 1.0)
/// * `mutation_rate: f64` - Maximum relative change in constant value
/// * `range_limits: Option<(f64, f64)>` - Optional min and max bounds for constants
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::mutation::ConstantMutation;
///
/// let mutation = ConstantMutation::default();
/// ```
pub struct ConstantMutation {
    probability: f64,
    mutation_rate: f64,
    range_limits: Option<(f64, f64)>
}

impl Default for ConstantMutation {
    fn default() -> Self {
        debug!("Creating default ConstantMutation with probability {}, mutation_rate {} and range_limits ({}, {})",
            0.1, 0.1, -1.0, 1.0
        );
        return Self::new(0.1, 0.1, Some((-1.0, 1.0))).expect("Failed to create default ConstantMutation");
    }
}

impl ConstantMutation {
    /// Creates new ConstantMutation operator.
    ///
    /// # Arguments
    /// * `probability: f64` - mutation probability (0.0 to 1.0)
    /// * `mutation_rate: f64` - maximum relative change in constant value
    /// * `range_limits: Option<(f64, f64)>` - optional min and max bounds
    ///
    /// # Returns
    /// * `Result<Self, MutationError>` - new operator or error if probability/rate invalid
    pub fn new(probability: f64, mutation_rate: f64, range_limits: Option<(f64, f64)>) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to create ConstantMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        if !(0.0..=1.0).contains(&mutation_rate) {
            error!("Attempted to create ConstantMutation with invalid mutation_rate: {}", mutation_rate);
            return Err(MutationError::InvalidMutationRate(mutation_rate));
        }
        if range_limits.is_none() {
            info!("Created ConstantMutation operator with probability {} and mutation_rate {}", 
                probability, mutation_rate
            );
            return Ok(Self { probability, mutation_rate, range_limits: None });
        }
        info!("Created ConstantMutation operator with probability {}, mutation_rate {} and range_limits ({}, {})", 
            probability, mutation_rate, range_limits.expect("Failed to extract min range").0, range_limits.expect("Failed to extract max range").1
        );
        return Ok(Self { probability, mutation_rate, range_limits });
    }
}

impl Mutator<TreeGenotype> for ConstantMutation {
    fn variate<R: Rng>(&self, rng: &mut R, individual: &TreeGenotype, sampler: &OperatorSampler) -> TreeGenotype {
        if rng.gen::<f64>() > self.probability {
            debug!("Skipping mutation..");
            return individual.clone();
        }
        let mut arena = individual.arena().clone();

        let constant_positions = arena.iter().enumerate()
            .filter(|(_, node)| {
                node.parse::<f64>().is_ok()
            })
            .map(|(i, _)| i).collect::<Vec<usize>>();

        if constant_positions.is_empty() {
            debug!("No constants to mutate! Skipping mutation..");
            return individual.clone();
        }

        let mutation_point = constant_positions[rng.gen_range(0..constant_positions.len())];

        let current_value = arena[mutation_point].parse::<f64>()
            .unwrap_or_else(|_| panic!("Failed to parse constant node: {}", arena[mutation_point]));
        let delta = 1.0 + (rng.gen::<f64>() * 2.0 - 1.0) * self.mutation_rate;
        let new_value = if let Some((min, max)) = self.range_limits {
            (current_value * delta).clamp(min, max)
        } else { current_value * delta };

        arena[mutation_point] = format!("{}", new_value);

        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);
        
        debug!("Completed mutation: constant {} -> {}", current_value, new_value);
        return tree;
    }
}
