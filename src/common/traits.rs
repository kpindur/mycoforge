//! Core evolution traits that define base building blocks for evolutionary algorithms.

use std::fmt::Display;

use rand::Rng;
use std::collections::HashMap;

use crate::operators::sampler::OperatorSampler;
use crate::common::types::VectorFunction;

/// Base trait for all genotypes in evolutionary algorithms.
///
/// Requires [`Clone`] and [`Display`][`std::fmt::Display`] implementations.
pub trait Genotype: Clone + Display {}

/// Handles initialization of new genotypes.
///
/// # Arguments
/// * `rng: &mut Rng` - random number generator, see [`Rng`][`rand::Rng`]
/// * `sampler: &OperatorSampler` - helper structure for sampling operators, see
///     [`OperatorSampler`][`crate::operators::sampler::OperatorSampler`]
///
/// # Returns
/// * `G` - newly initialized genotype
pub trait Initializer<G: Genotype> {
    fn initialize<R: Rng>(&self, rng: &mut R, sampler: &OperatorSampler) -> G;
}

/// Performs mutation operations on [`Genotype`][`crate::common::traits::Genotype`]
///
/// # Arguments
/// * `rng: &mut Rng` - random number generator, see [`Rng`][`rand::Rng`]
/// * `individual: &G` - [`Genotype`][`crate::common::traits::Genotype`] to variate
/// * `sampler: &OperatorSampler` - helper structure for sampling operators, see
///     [`OperatorSampler`][`crate::operators::sampler::OperatorSampler`]
///
/// # Returns
/// * `G` - mutated individual
pub trait Mutator<G: Genotype> {
    fn variate<R: Rng>(&self, rng: &mut R, individual: &G, sampler: &OperatorSampler) -> G;
}

/// Performs crossover operations on [`Genotype`][`crate::common::traits::Genotype`]
///
/// # Arguments
/// * `rng: &mut Rng` - random number generator, see [`Rng`][`rand::Rng`]
/// * `parent1: &G` - first [`Genotype`][`crate::common::traits::Genotype`] to variate
/// * `parent2: &G` - second [`Genotype`][`crate::common::traits::Genotype`] to variate
/// * `sampler: &OperatorSampler` - helper structure for sampling operators, see
///     [`OperatorSampler`][`crate::operators::sampler::OperatorSampler`]
///
/// # Returns
/// * `Vec<G>` - two crossed over individuals, first individual with subtree from the second
///     individual and second individual with subtree from the first individual
pub trait Crossoverer<G: Genotype> {
    fn variate<R: Rng>(&self, rng: &mut R, parent1: &G, parent2: &G, sampler: &OperatorSampler) -> Vec<G>;
}

/// Provides access to training and test datasets.
///
/// # Returns
/// * `data_train` - tuple of (number of samples, vector of features)
/// * `data_test` - tuple of (number of samples, vector of features)
pub trait Data {
    fn data_train(&self) -> (usize, &Vec<Vec<f64>>);
    fn data_test(&self)  -> (usize, &Vec<Vec<f64>>);
}

/// Evaluates fitness of genotypes, with optional memoization support.
///
/// # Arguments
/// * `tree: &G` - [`Genotype`][`crate::common::traits::Genotype`] to evaluate
/// * `data: &Self::D` - dataset implementing [`Data`][`crate::common::traits::Data`] trait
/// * `map: &HashMap<String, (usize, VectorFunction)>` - mapping of function names to their 
///     implementations, see [`VectorFunction`][`crate::common::types::VectorFunction`]
/// * `cache: &HashMap<G, f64>` - (memoized version only) cache of previously computed 
///     fitness values
///
/// # Returns
/// * `f64` - computed fitness value
pub trait Evaluator<G: Genotype> {
    type D: Data;

    fn evaluate(&self, 
        tree: &G, data: &Self::D, 
        map: &HashMap<String, (usize, VectorFunction)>
    ) -> f64;

    fn memoized_evaluate(&self, 
        tree: &G, data: &Self::D, 
        map: &HashMap<String, (usize, VectorFunction)>,
        cache: &HashMap<G, f64>
    ) -> f64;
}

/// Performs selection of genotypes from population.
///
/// # Arguments
/// * `rng: &mut Rng` - random number generator, see [`Rng`][`rand::Rng`]
/// * `population: &[Self::I]` - slice of individuals implementing 
///     [`Individual`][`crate::common::traits::Individual`]
///
/// # Returns
/// * `G` - selected [`Genotype`][`crate::common::traits::Genotype`]
pub trait Selector<G: Genotype> {
    type I: Individual<G>;

    fn select<R: Rng>(&self, rng: &mut R, population: &[Self::I]) -> G;
}

/// Represents an individual in population, combining genotype and its fitness.
///
/// # Methods
/// * `genotype()` - returns reference to underlying [`Genotype`][`crate::common::traits::Genotype`]
/// * `phenotype()` - returns fitness value
///
/// # Conversion Methods
/// * `from_vecs` - creates individuals from separate genotype and fitness vectors
/// * `from_genotype_vec` - creates individuals from genotypes (fitness needs to be computed)
/// * `to_genotype_vec` - extracts genotypes from individuals
///
/// # Returns
/// * Methods return either reference to genotype, fitness value, or vector of individuals
pub trait Individual<G: Genotype>: Sized {
    fn genotype(&self) -> &G;
    fn phenotype(&self) -> f64;

    fn from_vecs(genotypes: &[G], fitness: &[f64]) -> Vec<Self>;
    fn from_genotype_vec(genotypes: &[G]) -> Vec<Self>;
    fn to_genotype_vec(individuals: &[Self]) -> Vec<G>;
}

/// Main optimization interface for evolutionary algorithms.
///
/// # Arguments 
/// * `rng: &mut R` - random number generator
/// * `population_size: usize` - size of initial population (init_population only)
/// * `population: &[Self::I]` - current population implementing [`Individual`][`crate::common::traits::Individual`]
///
/// # Returns
/// * `Vec<G>` - vector of new [`Genotype`][`crate::common::traits::Genotype`]s
pub trait Optimizer<G: Genotype> {
    type I: Individual<G>;
    fn init_population<R: Rng>(&self, rng: &mut R, population_size: usize) -> Vec<G>;
    fn optimize<R: Rng>(&self, rng: &mut R, population: &[Self::I]) -> Vec<G>;
}
