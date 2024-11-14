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

    fn swap(parents: (&TreeGenotype, &TreeGenotype), crossover_points: (usize, usize)) 
        -> Vec<Vec<String>> {
        let (parent1, parent2) = parents;
        let (xo_point1, xo_point2) = crossover_points;

        let sub_end1 = parent1.subtree(xo_point1);
        let sub_end2 = parent2.subtree(xo_point2);

        let subtree1 = &parent1.arena()[xo_point1..=sub_end1];
        let subtree2 = &parent2.arena()[xo_point2..=sub_end2];

        let mut tree1 = parent1.arena()[..xo_point1].to_vec();
        tree1.extend_from_slice(subtree2);
        tree1.extend_from_slice(&parent1.arena()[sub_end1+1..]);

        let mut tree2 = parent2.arena()[..xo_point2].to_vec();
        tree2.extend_from_slice(subtree1);
        tree2.extend_from_slice(&parent2.arena()[sub_end2+1..]);

        return vec![tree1, tree2];
    }

}

impl Crossoverer<TreeGenotype> for SubtreeCrossover {
    fn variate<R: Rng>(&self, rng: &mut R, parent1: &TreeGenotype, parent2: &TreeGenotype, sampler: &OperatorSampler) -> Vec<TreeGenotype> {
        if rng.gen::<f64>() > self.probability { return [parent1.clone(), parent2.clone()].to_vec(); }

        let crossover_points: (usize, usize) = (rng.gen_range(0..parent1.arena().len()), rng.gen_range(0..parent2.arena().len()));
        let trees = Self::swap( (parent1, parent2), crossover_points);
        // Change arena.clone() to &arena in the future
        let mut mutants = Vec::new();
        for tree in trees {
            let mut child = TreeGenotype::with_arena(tree.clone());
            *child.children_mut() = child.construct_children(sampler);
            mutants.push(child);
        }

        return mutants;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::traits::Initializer;
    use crate::tree::operators::init::Grow;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_subtree_crossover() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);
        
        let mut rng = StdRng::seed_from_u64(42);

        let init_scheme = Grow::new(2, 4);
        let parent1 = init_scheme.initialize(&mut rng, &sampler);
        let parent2 = init_scheme.initialize(&mut rng, &sampler);
        
        let crossover = SubtreeCrossover::new(1.0);
        let children = crossover.variate(&mut rng, &parent1, &parent2, &sampler);

        assert_ne!(parent1.arena(), children[0].arena());
        assert!(!children[0].children().is_empty());
        assert_ne!(parent1.children(), children[0].children());
        
        assert_ne!(parent2.arena(), children[1].arena());
        assert!(!children[1].children().is_empty());
        assert_ne!(parent2.children(), children[1].children());
    }
}
