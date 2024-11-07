use std::collections::HashMap;

use rand::Rng;

use crate::common::traits::{Initializer, Mutator};
use crate::tree::core::{individual::TreeGenotype, sampler::OperatorSampler};

use super::init::Grow;

pub struct SubtreeMutation {
    probability: f64,
}

impl SubtreeMutation {
    pub fn new(probability: f64) -> Self {
        return Self { probability };
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
        if rng.gen::<f64>() > self.probability { return individual.clone(); }
        
        let mutation_point: usize = rng.gen_range(0..individual.arena().len());
        
        let init_scheme = Grow::new(0, 2);
        let subtree: TreeGenotype = init_scheme.initialize(rng, sampler);
        
        let arena = Self::substitute(individual, &subtree, mutation_point);
        let mut tree = TreeGenotype::with_arena(arena);
        *tree.children_mut() = tree.construct_children(sampler);

        return tree.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_subtree_mutation() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);
        
        let mut rng = StdRng::seed_from_u64(42);

        let init_scheme = Grow::new(1, 2);
        let tree = init_scheme.initialize(&mut rng, &sampler);
        
        let mutator = SubtreeMutation::new(1.0);
        let mutant = mutator.variate(&mut rng, &tree, &sampler);

        assert_ne!(tree.arena(), mutant.arena());
        assert_ne!(tree.children(), mutant.children());
    }
}
