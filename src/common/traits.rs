
pub trait Genotype {
    fn initialize(&mut self);
    fn clone(&self) -> Box<dyn Genotype>;
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
