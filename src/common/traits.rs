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

    fn evaluate(&self, tree: &G, data: &Self::D, map: HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)>) -> f64;
}

pub trait Selector<G: Genotype> {
    fn select(&self, population: &[G]) -> G;
}


pub trait Individual {
    type G: Genotype;

    fn new(genotype: Self::G) -> Self;
    fn genotype(&self) -> &Self::G;
    fn phenotype(&self) -> f64;
}
