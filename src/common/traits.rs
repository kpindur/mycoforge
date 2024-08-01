
pub trait Genotype {
    fn initialize(&mut self);
    fn clone(&self) -> Box<dyn Genotype>;
pub trait Initializer<G: Genotype> {
    fn initialize(&self) -> G;
}

pub trait Mutator<G: Genotype> {
    fn variate(&self, individual: &G) -> G;
}

pub trait Crossoverer<G: Genotype> {
    fn variate(&self, parent1: &G, parent2: &G) -> G;
}

pub trait Selector<G: Genotype> {
    fn select(&self, population: &[G]) -> G;
}

pub trait Phenotype {
    fn evaluate(&self) -> f64;
}

pub trait Individual {
    type G: Genotype;
    type P: Phenotype;

    fn new(genotype: Self::G) -> Self;
    fn genotype(&self) -> &Self::G;
    fn phenotype(&self) -> &Self::P;
}
