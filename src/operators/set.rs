type VectorFunction = fn(&[&[f64]]) -> Vec<f64>;

pub trait OperatorSet {
    fn get_operator(&self, name: &str) -> Option<&Functor>;
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

