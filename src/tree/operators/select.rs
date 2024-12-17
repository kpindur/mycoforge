//! Selection methods for Genetic Programming
//!
//! This module provides selection operators for GP algorithms designed for selecting individuals
//! based on their fitness values. Also serves as a template for custom selection operators.

use core::panic;

use log::error;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::common::traits::{Individual, Selector};
use crate::tree::core::{tree::TreeGenotype, individual::TreeIndividual};

/// Tournament selection operator that selects best individual from random subset.
///
/// # Fields
/// * `tournament_size: usize` - Number of individuals randomly sampled for tournament
///
/// # Examples
/// ```
/// use mycoforge::tree::operators::select::TournamentSelection;
///
/// let selection = TournamentSelection::new(7);
///
/// assert_eq!(7, selection.tournament_size(),
///     "Tournament sizes do not match! Expected {}, found {}",
///     7, selection.tournament_size()
/// );
/// ```
pub struct TournamentSelection {
    tournament_size: usize,
}

impl TournamentSelection {
    /// Creates new TournamentSelection operator.
    ///
    /// # Arguments
    /// * `tournament_size: usize` - number of individuals in tournament
    pub fn new(tournament_size: usize) -> Self { return Self { tournament_size }; }

    pub fn tournament_size(&self) -> usize { return self.tournament_size; }
}

impl Selector<TreeGenotype> for TournamentSelection {
    type I = TreeIndividual<TreeGenotype>;
    fn select<R: Rng>(&self, rng: &mut R, population: &[TreeIndividual<TreeGenotype>]) -> TreeGenotype {
        if self.tournament_size > population.len() {
            error!("Tournament size {} exceeds population size {}!", 
                self.tournament_size, population.len()
            );
            panic!("Tournament size {} exceeds population size {}!",
                self.tournament_size, population.len()
            );
        }

        return population.choose_multiple(rng, self.tournament_size)
            .min_by(|a, b| 
                a.phenotype().partial_cmp(&b.phenotype()
            ).unwrap_or_else(|| panic!("Fitness comparison failed: {} ? {}", a.phenotype(), b.phenotype())))
            .expect("Tournament selection failed!")
            .genotype().clone();
     }
}

