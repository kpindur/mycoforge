use std::fmt::Display;

use rand::Rng;

use crate::tree::core::sampler::OperatorSampler;

pub trait Genotype: Clone + Display {}

pub trait Initializer<G: Genotype> {
    fn initialize<R: Rng>(&self, rng: &mut R, sampler: &OperatorSampler) -> G;
}

pub trait Mutator<G: Genotype> {
    fn variate<R: Rng>(&self, rng: &mut R, individual: &G, sampler: &OperatorSampler) -> G;
}

pub trait Crossoverer<G: Genotype> {
    fn variate<R: Rng>(&self, rng: &mut R, parent1: &G, parent2: &G, sampler: &OperatorSampler) -> Vec<G>;
}

pub trait Selector<G: Genotype> {
    fn select(&self, population: &[G]) -> G;
}

pub trait Evaluator {
    fn evaluate(&self) -> f64;
}

pub trait Individual {
    type G: Genotype;
    type E: Evaluator;

    fn new(genotype: Self::G) -> Self;
    fn genotype(&self) -> &Self::G;
    fn phenotype(&self) -> &Self::E;
}
