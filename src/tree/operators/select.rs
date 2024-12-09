use core::panic;

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
        if self.tournament_size > population.len() {
            panic!("Tournament size {} exceeds population size {}!",
                self.tournament_size, population.len()
            );
        }

        return population.choose_multiple(rng, self.tournament_size)
            .min_by(|a, b| 
                a.phenotype().partial_cmp(&b.phenotype()
            ).expect(&format!("Fitness comparison failed: {} ? {}", a.phenotype(), b.phenotype())))
            .expect("Tournament selection failed!")
            .genotype().clone();
     }
}

