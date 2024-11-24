use std::fmt::Display;

use rand::Rng;
use std::collections::HashMap;

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

pub trait Data {
    fn data_train(&self) -> (usize, &Vec<Vec<f64>>);
    fn data_test(&self)  -> (usize, &Vec<Vec<f64>>);
}

pub trait Evaluator<G: Genotype> {
    type D: Data;

    fn evaluate(&self, tree: &G, data: &Self::D, map: &HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)>) -> f64;
}

pub trait Selector<G: Genotype> {
    type I: Individual<G>;

    fn select<R: Rng>(&self, rng: &mut R, population: &[Self::I]) -> G;
}

pub trait Individual<G: Genotype>: Sized {
    fn genotype(&self) -> &G;
    fn phenotype(&self) -> f64;

    fn from_vecs(genotypes: &[G], fitness: &[f64]) -> Vec<Self>;
    fn from_genotype_vec(genotypes: &[G]) -> Vec<Self>;
    fn to_genotype_vec(individuals: &[Self]) -> Vec<G>;
}

pub trait Optimizer<G: Genotype> {
    type I: Individual<G>;
    fn init_population<R: Rng>(&self, rng: &mut R, population_size: usize) -> Vec<G>;
    fn optimize<R: Rng>(&self, rng: &mut R, population: &[Self::I]) -> Vec<G>;
}
