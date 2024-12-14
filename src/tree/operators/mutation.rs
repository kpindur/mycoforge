use rand::Rng;

use crate::common::traits::{Initializer, Mutator};
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::OperatorSampler;

use super::init::Grow;

use super::errors::MutationError;
use log::{info, error, debug};

fn substitute(individual: &TreeGenotype, subtree: &TreeGenotype, mutation_point: usize) 
    -> Vec<String> {
    let mutation_end: usize = individual.subtree(mutation_point);

    let mut new_arena = individual.arena()[0..mutation_point].to_vec();
    new_arena.extend(subtree.arena().iter().cloned());
    new_arena.extend(individual.arena()[mutation_end+1..].iter().cloned());

    return new_arena;
}

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
