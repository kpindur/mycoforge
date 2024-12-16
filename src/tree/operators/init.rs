//! Tree initialization methods for Genetic Programming.
//!
//! Provides standard implementations: Grow and Full methods.
//! Designed to return `TreeGenotype` structure. Does not include `Ramped Half and Half` method,
//! because it's more of method of constructing population, not a single individual.
//! Serves as a template for creating custom initialization methods.

use std::collections::HashMap;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::common::traits::Initializer;
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::{OperatorSampler, Sampler};

/// Grow initialization method
///
/// Generates trees with depths between minimum and maximum bounds.
/// Forces function nodes until min_depth, then randomly selects between terminals/functions.
///
/// # Fields:
/// * `min_depth: usize` - minimum depth of a tree
/// * `max_depth: usize` - maximum depth of a tree
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::init::Grow;
/// 
/// let initializer = Grow::new(1, 2); // min_depth=1, max_depth=2
/// ```
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

/// Full initialization method.
///
/// Generates complete trees where all branches reach specified depth.
/// Implemented as a special case of Grow with min_depth = max_depth.
///
/// # Fields:
/// `depth: usize` - depth of the tree
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::init::Full;
///
/// let init = Full::new(2); // depth=2
/// ```
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
