//! Core individual structure for evolutionary algorithms.
//!
//! This module provides the [`TreeIndividual`] structure that combines genotype with its fitness
//! value.
use crate::common::traits::{Genotype, Individual};

/// Individual representation that pairs genotype with its fitness value.
///
/// # Type Parameters
/// * `G: Genotype`- type implementing [`Genotype`][`crate::common::traits::Genotype`] trait
///
/// # Fields
/// * `genotype: G` - [`Genotype`][`crate::common::traits::Genotype`] representation
/// * `fitness: f64` - fitness value
///
/// # Examples
/// ```
/// use mycoforge::common::traits::Individual;
/// use mycoforge::tree::core::individual::TreeIndividual;
/// use mycoforge::tree::core::tree::TreeGenotype;
///
/// let individual = TreeIndividual::new(TreeGenotype::default(), 0.0);
///
/// assert!(individual.genotype().arena().is_empty() &&
///     individual.genotype().children().is_empty(),
///     "Default individual should be empty!"
/// );
/// assert!(0.0 - individual.phenotype() < 1e-10,
///     "Default individuals fitness should be {}, found {}",
///     0.0, individual.phenotype()
/// );
/// ```
#[derive(Clone)]
pub struct TreeIndividual<G: Genotype> {
    genotype: G,
    fitness: f64
}

impl<G: Genotype> TreeIndividual<G> {
    /// Creates new individual with given genotype and fitness.
    ///
    /// # Arguments
    /// * `genotype: G` - genotype representation
    /// * `fitness: f64` - fitness value
    pub fn new(genotype: G, fitness: f64) -> Self {
        return Self { genotype, fitness };
    }
}

impl<G: Genotype> Individual<G> for TreeIndividual<G> {
    fn genotype(&self) -> &G { return &self.genotype; }
    fn phenotype(&self) -> f64 { return self.fitness; }

    fn from_vecs(genotypes: &[G], fitness: &[f64]) -> Vec<Self> {
        return genotypes.iter().zip(fitness.iter()).map(|(g, &f)| Self::new(g.clone(), f)).collect();
    }
    fn from_genotype_vec(genotypes: &[G]) -> Vec<Self> {
        return genotypes.iter().map(|g| Self::new(g.clone(), f64::NEG_INFINITY)).collect();
    }
    fn to_genotype_vec(individuals: &[Self]) -> Vec<G> {
        return individuals.iter().map(|i| i.genotype().clone()).collect();
    }
}
