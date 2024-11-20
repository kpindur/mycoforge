use rand::Rng;
use rand::seq::SliceRandom;

use crate::common::traits::{Individual, Selector};
use crate::tree::core::{tree::TreeGenotype, individual::TreeIndividual};

pub struct TournamentSelection {
    tournament_size: usize,
}

impl TournamentSelection {
    pub fn new(tournament_size: usize) -> Self { return Self { tournament_size }; }
}

impl Selector<TreeGenotype> for TournamentSelection {
    type I = TreeIndividual<TreeGenotype>;
    fn select<R: Rng>(&self, rng: &mut R, population: &[TreeIndividual<TreeGenotype>]) -> TreeGenotype {
        return population.choose_multiple(rng, self.tournament_size)
            .max_by(|a, b| a.phenotype().partial_cmp(&b.phenotype())
                .unwrap()).unwrap().genotype().clone();
     }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::traits::Initializer;
    use crate::tree::operators::init::Grow;
    use crate::tree::core::sampler::OperatorSampler;
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
        let mut population = Vec::new();
        for i in 0..10 {
            let genotype = init_scheme.initialize(&mut rng, &sampler);
            let fitness = i as f64;
            population.push(TreeIndividual::new(genotype, fitness));
        }
        
        let selection = TournamentSelection::new(5);
        let chosen = selection.select(&mut rng, &population);
        println!("{}", chosen);
    }
}
