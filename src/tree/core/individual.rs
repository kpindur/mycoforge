
use crate::common::traits::{Genotype, Individual};

pub struct TreeIndividual<G: Genotype> {
    genotype: G,
    fitness: f64
}

impl<G: Genotype> TreeIndividual<G> {
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
