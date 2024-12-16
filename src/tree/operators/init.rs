use std::collections::HashMap;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::common::traits::Initializer;
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::{OperatorSampler, Sampler};

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
        
        let (term_set, func_set) = (
            sampler.sampler_with_arity(0, 0), 
            sampler.sampler_with_arity(1, *sampler.arities().iter().max().expect("Failed to get highest arity!"))
        );

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

pub struct Full {
    depth: usize
}

impl Full {
    pub fn new(depth: usize) -> Self { return Self { depth }; }
}

impl Initializer<TreeGenotype> for Full {
    fn initialize<R: Rng>(&self, rng: &mut R, sampler: &OperatorSampler) -> TreeGenotype {
        let scheme = Grow::new(self.depth, self.depth);
        return scheme.initialize(rng, sampler);
    }
}
