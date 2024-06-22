use rand::RngCore;

use crate::genotype::init::{
    InitUniform, InitFromDistribution
};

pub enum Initialization<T> {
    Uniform(InitUniform<T>),
    FromDistribution(InitFromDistribution<T>),
}


use crate::genotype::mutation::{
    UniformBinaryMutation,
};

pub enum Mutation<T> {
    UniformBinary(UniformBinaryMutation),
    marker(std::marker::PhantomData<T>)
}

/// Crossover trait
/// Should crossover the parents and return a tuple. That is, crossover should be defined over
/// the chosen genotype. `<T>` refers to a Genotype, for example `<T>: Vec<bool>`.
pub trait Crossover<R, T> 
where
    R: RngCore
{
    fn crossover(&self, rng:&mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>>;
}

/// Selection trait
/// Should select specified number of individuals from given population. 
pub trait Selection<R, T>
where
    R: RngCore
{
    fn select(&self, rng: &mut R, population: Vec<Vec<T>>) -> Vec<Vec<T>>;
}

/// Genotype trait, which requires users to define three methods for custom genotypes:
/// initialize, mutate, and crossover
pub trait Genotype<R, T>
where
    R: RngCore,
    Self: Sized
{
    fn initialize(rng: &mut R, init_scheme: &Initialization<T>) -> Self;
    fn mutate(&self, rng: &mut R, mutation_scheme: &Mutation<T>) -> Self;
    fn crossover(&self, rng: &mut R, other: &Self, crossover_scheme: &impl Crossover<R, T>) -> Vec<Self>;
}
