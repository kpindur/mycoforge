
pub trait Genotype {
    fn new() -> Self;
    // Other methods
}

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
