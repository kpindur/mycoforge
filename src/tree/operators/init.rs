use std::collections::HashMap;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::common::traits::Initializer;
use crate::tree::core::individual::TreeGenotype;
use crate::tree::core::sampler::{OperatorSampler, Sampler};

pub struct Grow {
    min_depth: usize,
    max_depth: usize
}

impl Grow {
    pub fn new(min_depth: usize, max_depth: usize) -> Self {
        return Self { min_depth, max_depth };
    }
}

impl Initializer<TreeGenotype> for Grow {
    fn initialize<R: Rng>(&self, rng: &mut R, sampler: &OperatorSampler) -> TreeGenotype {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut tree: TreeGenotype = TreeGenotype::new(Vec::new(), HashMap::new());
        
        let (term_set, func_set) = (sampler.sampler_with_arity(0, 0), sampler.sampler_with_arity(1, 2)); // max_arity should depend on sample, e.g., sampler.arity().iter().max()

        let mut root: usize = 0;
        let (node_id, node_arity) = 
            if self.max_depth == 0 {
                term_set.sample(rng)
            } else {
                func_set.sample(rng)
            };

        tree.arena_mut().push(node_id);
        for _ in 0..node_arity {
            stack.push((root, 1));
        }
        
        while let Some((parent, depth)) = stack.pop() {
            root += 1;
            let (node_id, node_arity) = 
                if depth == self.max_depth {
                    term_set.sample(rng)
                } else if depth < self.min_depth {
                    func_set.sample(rng)
                } else {
                    [&term_set, &func_set].choose(rng).unwrap().sample(rng)
                };

            tree.arena_mut().push(node_id);
            tree.children_mut().entry(parent).or_default().push(root);
            for _ in 0..node_arity {
                stack.push((root, depth+1));
            }
        }

        return tree;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn tmp_valid_tree(tree: TreeGenotype) -> bool {
        let mut result: usize = 0;
        for (_, value) in tree.children() {
            result += value.len();
        }

        if (result + 1) != tree.arena().len() {
            return false;
        }
        return true;
    }

    #[test]
    fn test_intializer_grow() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);
        
        let mut rng = StdRng::seed_from_u64(42);

        let init_scheme = Grow::new(1, 2);
        let tree = init_scheme.initialize(&mut rng, &sampler);

        assert!(tmp_valid_tree(tree));
    }
}
