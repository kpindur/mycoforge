
pub mod linear_structure {
    use crate::genotype::traits::Initialization;
    use rand::{
        Rng, RngCore,
        distributions::{Distribution, Standard, WeightedIndex}, 
    };

    /// Initialize linear structure uniformly
    /// 
    /// This initialization scheme creates a vector of elements with uniformly 
    /// distributed values of `T`.
    pub struct InitUniform<T> {
        size: usize,
        _marker: std::marker::PhantomData<T>
    }

    impl<T> InitUniform<T> {
        /// Creates a new instance of `InitUniform`.
        ///
        /// # Arguments
        ///
        /// * `size`: The size of the vector to be initialized
        ///
        /// # Returns
        ///
        /// A new instance of `InitUniform`.
        pub fn new(size: usize) -> Self {
            return InitUniform{ size, _marker: std::marker::PhantomData }
        }
    }

    impl<R, T> Initialization<R, T> for InitUniform<T>
    where
        R: RngCore,
        Standard: Distribution<T>
    {
        /// Initialize linear structure uniformly. 
        ///
        /// # Arguments
        ///
        /// * `rng`: mutable random number generator
        ///
        /// # Returns
        ///
        /// Vector of randomly initialized elements.
        fn initialize(&self, rng: &mut R) -> Vec<T> {
            let seq: Vec<T> = (0..self.size).map(|_| rng.gen::<T>()).collect();

            return seq;
        }
    }

    /// Initialize linear structure according to a given set of probability distributions
    /// 
    /// This initialization scheme creates a vector of elements distributed according to a set of
    /// probability distributions.
    pub struct InitFromDistribution<T> {
        size: usize,
        range: Vec<T>,
        dist: Vec<WeightedIndex<f64>>,
    }
    
    impl<T> InitFromDistribution<T>
    where
        T: Clone
    {
        /// Create an instance of `InitFromDistribution`.  
        ///
        /// # Arguments
        ///
        /// * `size`: The size of the initialized individual
        /// * `range`: The possible values over which probability distributions are defined
        /// * `distributions`: Set of probability distributions of values in `range` at specific indices.
        /// E.g. `distributions[i]` is the distribution of `range` at the index `i`
        ///
        /// # Returns
        ///
        /// A new instance of `InitFromDistribution`.
        fn new(size: usize, range: &[T], distributions: &[&[f64]]) -> Self {
            let mut dists: Vec<WeightedIndex<f64>> = Vec::new();

            for &distribution in distributions {
                assert_eq!(range.len(), distribution.len(), "Error: Lengths do not match!");
                let is_distribution = distribution.iter().sum::<f64>() == 1.0;
                assert!(is_distribution, 
                        "Error: Probability distribution does not sum to 1.0! Sum: {}", 
                        distribution.iter().sum::<f64>());
                dists.push(WeightedIndex::new(distribution).unwrap());
            }

            return InitFromDistribution {
                size, range: range.to_vec(), dist: dists
            };
        }
    }

    impl<R, T> Initialization<R, T> for InitFromDistribution<T> 
    where
        R: RngCore,
        T: Clone,
        Standard: Distribution<T>
    {
        /// Initialize linear structure. 
        ///
        /// # Arguments
        ///
        /// * `rng` - mutable random number generator
        ///
        /// # Returns
        ///
        /// Vector of randomly initialized elements.
        fn initialize(&self, rng: &mut R) -> Vec<T> {
            let seq: Vec<usize> = (0..self.size).zip(self.dist.iter())
                .map(|(_, dist)| dist.sample(rng)).collect();

            let seq: Vec<T> = seq.iter().map(|&id| self.range[id].clone() ).collect();

            return seq;
        }
    }
    
    #[cfg(test)]
    mod linear_tests {
        use super::*;
        use std::collections::HashMap;
        use rand::{
            rngs::StdRng,
            SeedableRng
        };

        fn chi_square_test(observed: &HashMap<usize, (usize, usize)>, expected: &HashMap<usize, (f64, f64)>) -> Vec<f64> {
            let mut results: Vec<f64> = Vec::new();

            for (index, (count_0, count_1)) in observed {
                let total_count = count_0 + count_1;

                if let Some((prob_0, prob_1)) = expected.get(index) {
                    let expected_count_0 = total_count as f64 * prob_0;
                    let expoected_count_1 = total_count as f64 * prob_1;

                    let mut chi_square = (*count_0 as f64 - expected_count_0).powi(2) / expected_count_0;
                    chi_square +=  (*count_1 as f64 - expoected_count_1).powi(2) / expoected_count_1;

                    results.push(chi_square);
                }
            }

            return results;
        }

        #[test]
        fn init_uniform_works() {
            let n: usize = 10;
            let init_scheme: InitUniform<bool> = InitUniform::new(n);

            let seed: [u8; 32] = [0; 32];
            let mut rng = StdRng::from_seed(seed);

            let m: usize = 100000;
            let samples: Vec<Vec<bool>> = (0..m).map(|_| init_scheme.initialize(&mut rng)).collect();
            // hashmap stores keys = index and values = (count_0, count_1)
            let mut observed: HashMap<usize, (usize, usize)> = HashMap::new();
            
            for sample in &samples {
                for (index, &val) in sample.iter().enumerate() {
                    if val {
                        let count = observed.entry(index).or_insert((0, 0)).clone(); 
                        observed.insert(index, (count.0, count.1 + 1));
                    }
                    if !val {
                        let count = observed.entry(index).or_insert((0, 0)).clone(); 
                        observed.insert(index, (count.0 + 1, count.1));
                    }
                }
            }
            
            let expected: HashMap<usize, (f64, f64)> = (0..n).map(|id| (id, (0.5f64, 0.5f64))).collect();
            let results = chi_square_test(&observed, &expected);
            // Critical value for chi-square test for 1-degree of freedom ~= 3.84
            assert!(results.iter().all(|&result| result < 3.84), "Error: Chi Square Test failed! {:?} > {}", results, 3.84)
        }

        fn vector_chi_square_test<T>(
            observed: &Vec<HashMap<T, usize>>, 
            expected: &Vec<HashMap<T, f64>>
        ) -> Vec<f64> 
        where
            T: Eq + std::hash::Hash + Copy
        {
            let mut results: Vec<f64> = Vec::new();

            for (index, observed_counts) in observed.iter().enumerate() {
                if let Some(expected_probs) = expected.get(index) {
                    let total_count: usize = observed_counts.values().sum();
                    let mut chi_square = 0.0;

                    for (value, expected_prob) in expected_probs {
                        let observed_count = *observed_counts.get(value).unwrap_or(&0);
                        let expected_count = total_count as f64 * expected_prob;

                        chi_square += (observed_count as f64 - expected_count).powi(2) / expected_count;
                    }

                    results.push(chi_square);
                }
            }

            return results;
        }

        #[test]
        fn init_from_distribution_works() {
            let n: usize = 10;
            let range: Vec<bool> = vec![false, true];
            let dists: Vec<Vec<f64>> = vec![
                vec![0.2, 0.8], vec![0.5, 0.5], vec![0.6, 0.4], vec![0.6, 0.4], vec![0.2, 0.8],
                vec![0.9, 0.1], vec![0.4, 0.6], vec![0.2, 0.8], vec![0.7, 0.3], vec![0.5, 0.5]];
            let dists: Vec<&[f64]> = dists.iter().map(|row| row.as_slice()).collect();
            let init_scheme: InitFromDistribution<bool> = InitFromDistribution::new(n, &range, &dists);

            let seed: [u8; 32] = [0; 32];
            let mut rng = StdRng::from_seed(seed);

            let m: usize = 100000;
            let samples: Vec<Vec<bool>> = (0..m).map(|_| init_scheme.initialize(&mut rng)).collect();
            // hashmap stores keys = index and values = (count_0, count_1)
            let mut observed: Vec<HashMap<bool, usize>> = vec![HashMap::new(); n];
            
            for sample in &samples {
                for (loci, &val) in sample.iter().enumerate() {
                    for &possible_val in &range {
                        if val != possible_val { continue; }
                        *observed[loci].entry(val).or_insert(0) += 1;
                    }
                }
            }
            
            let expected: Vec<HashMap<bool, f64>> = (0..n)
                .map(|id| {
                    let counts = range.iter().cloned().zip(dists[id].iter().cloned()).collect();
                    return counts;
                }).collect();
            let results = vector_chi_square_test(&observed, &expected);
            // Critical value for chi-square test for 1-degree of freedom ~= 3.84
            assert!(results.iter().all(|&result| result < 3.84), "Error: Chi Square Test failed! {:?} > {}", results, 3.84)
            
        }
    }
}
