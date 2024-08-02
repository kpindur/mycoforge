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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_weights() {
        let operators = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let arity = vec![2, 0, 0];
        let initial_weights = vec![1.0, 2.0, 3.0];
        let mut sampler = OperatorSampler::new(operators, arity, initial_weights);

        let new_weights = vec![2.0, 1.0, 3.0];
        sampler.update_weights(new_weights.clone());

        assert_eq!(sampler.weights, new_weights);
    }

    #[test]
    fn test_sampler_with_arity() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);

        let external = sampler.sampler_with_arity(0, 0);
        let internal = sampler.sampler_with_arity(1, 2);
        let internal_one = sampler.sampler_with_arity(1, 1);
        let internal_two = sampler.sampler_with_arity(2, 2);

        assert_eq!(3, external.operators.len());
        assert_eq!(3, internal.operators.len());
        assert_eq!(1, internal_one.operators.len());
        assert_eq!(2, internal_two.operators.len());
    }

    fn test_operator_sampler_distribution() {
        let operators = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let arity = vec![2, 0, 0];
        let weights = vec![1.0, 2.0, 3.0];
        
        let sampler = OperatorSampler::new(operators, arity, weights);
        
        let mut rng = StdRng::seed_from_u64(42);
        let n_samples = 1000;
        let mut observed = [0; 3];

        for _ in 0..n_samples {
            let sample = sampler.sample(&mut rng);
            match sample.0.as_str() {
                "A" => observed[0] += 1,
                "B" => observed[1] += 1,
                "C" => observed[2] += 1,
                _ => panic!("Unexpected sample"),
            }
        }

        let expected = [
            n_samples as f64 * (1.0 / 6.0),
            n_samples as f64 * (2.0 / 6.0),
            n_samples as f64 * (3.0 / 6.0),
        ];

        let chi_square: f64 = observed.iter().zip(expected.iter())
            .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
            .sum();
        
        // Degrees of freedom: 3 - 1 = 2
        // For 95% confidence and 2 degrees of freedom, critical value is about 5.991
        assert!(chi_square < 5.991, "Chi-square test failed");
    }
}
