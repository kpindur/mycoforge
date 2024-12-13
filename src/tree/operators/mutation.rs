use rand::Rng;

use crate::common::traits::{Initializer, Mutator};
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::OperatorSampler;

use super::init::Grow;

use super::errors::MutationError;
use log::{info, error, debug};

pub struct SubtreeMutation {
    probability: f64,
}

impl SubtreeMutation {
    pub fn new(probability: f64) -> Result<Self, MutationError> {
        if !(0.0..=1.0).contains(&probability) {
            error!("Attempted to crate SubtreeMutation with invalid probability: {}", probability);
            return Err(MutationError::InvalidProbability(probability));
        }
        info!("Created SubtreeMutation operator with probability {}", probability);
        return Ok(Self { probability });
    }

    fn substitute(individual: &TreeGenotype, subtree: &TreeGenotype, mutation_point: usize) 
        -> Vec<String> {
        let mutation_end: usize = individual.subtree(mutation_point);

        let mut new_arena = individual.arena()[0..mutation_point].to_vec();
        new_arena.extend(subtree.arena().iter().cloned());
        new_arena.extend(individual.arena()[mutation_end+1..].iter().cloned());

        return new_arena;
    }
}

impl Mutator<TreeGenotype> for SubtreeMutation {
    fn variate<R: Rng>(&self, rng: &mut R, individual: &TreeGenotype, sampler: &OperatorSampler) -> TreeGenotype {
        if rng.gen::<f64>() > self.probability { 
            debug!("Skipping mutation..");
            return individual.clone(); 
        }
        
        let mutation_point: usize = rng.gen_range(0..individual.arena().len());
        
        let init_scheme = Grow::new(0, 2);
        let subtree: TreeGenotype = init_scheme.initialize(rng, sampler);
        debug!("Generated subtree of size {} at point {}", subtree.arena().len(), mutation_point);
        
        let arena = Self::substitute(individual, &subtree, mutation_point);
        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);
        
        debug!("Completed mutation: original size {} -> mutant size {}", individual.arena().len(), tree.arena().len());
        return tree.clone();
    }
}
