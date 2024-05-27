use rand::RngCore;

/// Initialization trait
/// Should initialize and return the object. That is, initialization should be defined over the chosen
/// genotype. <T> refers to a type cncapsulated in Genotype, s.t. <T> -> Vec<T>, for example
/// Initialization<bool> -> Vec<bool>.
pub trait Initialization<R, T> 
where
    R: RngCore
{
    fn initialize(&self, rng: &mut R) -> Vec<T>;
}

/// Mutation trait
/// Should mutate and return the object. That is, mutation should be defined over the chosen genotype. <T>
/// refers to a Genotype, for example <T>: Vec<bool>.
pub trait Mutation<R, T>
where
    R: RngCore
{
    fn mutate(&self, rng: &mut R, genotype: &[T]) -> Vec<T>;
}

/// Crossover trait
/// Should crossover the parents and return a tuple. That is, crossover should be defined over
/// the chosen genotype. <T> refers to a Genotype, for example <T>: Vec<bool>.
pub trait Crossover<R, T> 
where
    R: RngCore
{
    fn crossover(&self, rng:&mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>>;
}

/// Genotype trait, which requires users to define three methods for custom genotypes:
/// initialize, mutate, and crossover
pub trait Genotype<R, T>
where
    R: RngCore,
    Self: Sized
{
    fn initialize(rng: &mut R, init_scheme: &impl Initialization<R, T>) -> Self;
    fn mutate(&self, rng: &mut R, mutation_scheme: &impl Mutation<R, T>) -> Self;
    fn crossover(&self, other: &Self, crossover_scheme: &impl Crossover<R, T>) -> Vec<Self>;
}
