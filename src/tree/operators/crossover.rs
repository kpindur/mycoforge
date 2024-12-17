use rand::Rng;

use crate::common::traits::Crossoverer;
use crate::tree::core::tree::TreeGenotype;
use crate::operators::sampler::OperatorSampler;

pub struct SubtreeCrossover {
    probability: f64,
}

impl Default for SubtreeCrossover {
    fn default() -> Self {
        debug!("Creating default SubtreeCrossover with probability {}", 0.7);
        return Self::new(0.7).expect("Failed to create default SubtreeCrossover!")
    }
}

    pub fn new(probability: f64) -> Result<Self, CrossoverError> {
        if !(0.0..=1.0).contains(&probability) { 
            error!("Attempted to crate SubtreeCrossover with invalid probability: {}", probability);
            return Err(CrossoverError::InvalidProbability(probability));
        }
        return Ok(Self { probability });
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
