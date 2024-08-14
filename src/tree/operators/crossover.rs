use std::collections::HashMap;

use rand::Rng;

use crate::common::traits::Crossoverer;
use crate::tree::core::{individual::TreeGenotype, sampler::OperatorSampler};

pub struct SubtreeCrossover {
    probability: f64,
}

impl SubtreeCrossover {
    pub fn new(probability: f64) -> Self {
        return Self { probability };
    }

    fn _swap(individual: &TreeGenotype, subtree: &TreeGenotype, mutation_point: usize) 
        -> Vec<(Vec<String>, HashMap<usize, Vec<usize>>)> {
        let subtree 
        return Vec::new();
    }
}

impl Crossoverer<TreeGenotype> for SubtreeCrossover {
    fn variate<R: Rng>(&self, rng: &mut R, parent1: &TreeGenotype, parent2: &TreeGenotype, _sampler: &OperatorSampler) -> Vec<TreeGenotype> {
        if rng.gen::<f64>() > self.probability { return [parent1.clone(), parent2.clone()].to_vec(); }
        return [parent1.clone(), parent2.clone()].to_vec();
//        let crossover_point: usize = rng.gen_range(0..parent1.arena().len());
//        
//        let init_scheme = Grow::new(0, 2);
//        let subtree: TreeGenotype = init_scheme.initialize(rng, sampler);
//        
//        let tree = SubtreeCrossover::substitute(parent1, &subtree, crossover_point);
//        
//        let tree = TreeGenotype::new(tree.0, tree.1);
//
//        return tree.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_subtree_mutation() {
        todo!()
    }
}
