use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub trait Sampler {
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

pub struct OperatorSampler {
    operators: Vec<String>,
    arity:     Vec<usize>,
    weights:   Vec<f64>,
}

impl OperatorSampler {
    pub fn new(operators: Vec<String>, arity: Vec<usize>, weights: Vec<f64>) -> Self {
        return Self { operators, arity, weights };
    }

    pub fn operators(&self) -> &Vec<String> { return &self.operators; }
    pub fn arities(&self) -> &Vec<usize> { return &self.arity; }
    pub fn weights(&self) -> &Vec<f64> { return &self.weights; }

    pub fn update_weights(&mut self, weights: Vec<f64>) {
        assert_eq!(self.weights.len(), weights.len());
        self.weights = weights;
    }

    pub fn sampler_with_arity(&self, min_arity: usize, max_arity: usize) -> OperatorSampler {
        let is_valid = |arity| -> bool {
            return arity >= min_arity && arity <= max_arity;
        };
        let (mut filtered_operators, mut filtered_arity, mut filtered_weights) = (Vec::new(), Vec::new(), Vec::new());
        for (i, &arity) in self.arity.iter().enumerate() {
            if is_valid(arity) {
                filtered_operators.push(self.operators[i].clone());
                filtered_arity.push(self.arity[i]);
                filtered_weights.push(self.weights[i]);
            }
        }
        return Self { operators: filtered_operators, arity: filtered_arity, weights: filtered_weights };
    }
}

impl Sampler for OperatorSampler {
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize) {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        let index: usize = dist.sample(rng);

        return (self.operators[index].clone(), self.arity[index]);
    }
}
