use std::collections::HashMap;
use rand::Rng;

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
    fn initialize<R: Rng>(&self, rng: &mut R, sampler: OperatorSampler) -> TreeGenotype {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut tree: TreeGenotype = TreeGenotype::new(Vec::new(), HashMap::new());
        
        let mut root: usize = 0;
        let (node_id, node_arity) = sampler.sample(rng);

        tree.arena_mut().push(node_id);
        for _ in 0..node_arity {
            stack.push((root, 1));
        }
        
        while let Some((parent, depth)) = stack.pop() {
            root += 1;
            let (node_id, node_arity) = sampler.sample(rng);

            tree.arena_mut().push(node_id);
            tree.children_mut().entry(parent).or_default().push(root);
            for _ in 0..node_arity {
                stack.push((root, depth+1));
            }
        }

        return tree;
    }
}
