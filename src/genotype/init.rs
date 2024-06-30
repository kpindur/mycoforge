use rand::{
    Rng, RngCore,
    distributions::{Distribution, Standard, WeightedIndex}, 
};

/// Initialize linear structure uniformly
/// 
pub struct InitUniform<T> {
    size: usize,
    _marker: std::marker::PhantomData<T>
}

impl<T> InitUniform<T> 
where
    Standard: Distribution<T>
{
    pub fn new(size: usize) -> Self {
        return InitUniform{ size, _marker: std::marker::PhantomData }
    }

    /// Initialize linear structure uniformly. 
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    ///
    /// # Returns
    ///
    /// Initialized Vec<T>.
    pub fn initialize<R: RngCore>(&self, rng: &mut R) -> Vec<T> {
        let seq: Vec<T> = (0..self.size).map(|_| rng.gen::<T>()).collect();

        return seq;
    }
}

/// Initialize linear structure with probability distribution
/// 
pub struct InitFromDistribution<T> {
    size: usize,
    range: Vec<T>,
    dist: Vec<WeightedIndex<f64>>,
}

impl<T> InitFromDistribution<T>
where
    T: Clone,
    Standard: Distribution<T>
{
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

    /// Initialize linear structure using stored probability distribution. 
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    ///
    /// # Returns
    ///
    /// Initialized Vec<T>.
    pub fn initialize<R: RngCore>(&self, rng: &mut R) -> Vec<T> {
        let seq: Vec<usize> = (0..self.size).zip(self.dist.iter())
            .map(|(_, dist)| dist.sample(rng)).collect();

        let seq: Vec<T> = seq.iter().map(|&id| self.range[id].clone() ).collect();

        return seq;
    }
}

#[cfg(test)]         
mod linear_tests {
    use super::*;
    use rand::{
        SeedableRng,
        rngs::StdRng,
    };
    use std::collections::HashMap;

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
        // Critical value for chi-square test for 1-degree od freedom ~= 3.84
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
        // Critical value for chi-square test for 1-degree od freedom ~= 3.84
        assert!(results.iter().all(|&result| result < 3.84), "Error: Chi Square Test failed! {:?} > {}", results, 3.84)
        
    }
}

use crate::genotype::genotype::operator_set_sampler::OperatorSampler;

///
///
pub struct Full<T> 
where
    T: Clone
{
    max_height: usize,
    func_set: OperatorSampler<T>,
    term_set: OperatorSampler<T>,
}

impl<T> Full<T> 
where
    T: PartialEq + Clone
{
    fn new(max_height: usize, 
           func_set: OperatorSampler<T>, term_set: OperatorSampler<T>) 
        -> Self {
        return Self { max_height, func_set, term_set };
    }

    /// Initialize full tree structure using stored probability distribution. 
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    /// * `depth` - current recursion depth
    /// * `stack` - mutable vector holding all generated values (id, depth, arity)
    ///
    /// # Returns
    ///
    /// Initialized Vec<(String, usize, usize)>.
    fn initialize<R: RngCore>
        (&self, rng: &mut R, 
         depth: usize, stack: &mut Vec<(String, usize, usize)>)//NOTE: How to make it parallelizable?
    {
        if depth == self.max_height {
            let (id, _, arity) = self.term_set.sample(rng);
            stack.push((id, depth, arity));
        } else {
            let(id, _, arity) = self.func_set.sample(rng);
            stack.push((id, depth, arity));
            for _ in 0..arity {
                self.initialize(rng, depth+1, stack);
            }
        }
    }
}


pub struct Grow<T> 
where
    T: Clone
{
    min_height: usize,
    max_height: usize,
    func_set: OperatorSampler<T>,
    term_set: OperatorSampler<T>
}

impl<T> Grow<T>
where
    T: PartialEq + Clone
{
    pub fn new(min_height: usize, max_height: usize, 
               func_set: OperatorSampler<T>, term_set: OperatorSampler<T>) 
        -> Self {
        return Self { min_height, max_height, func_set, term_set };
    }
    /// Grow tree structure using stored probability distribution
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    /// * 'depth' - current recursion depth
    /// * 'stack' - vector to store initialized tree
    ///
    /// # Returns
    ///
    /// Initialized Vec<(String, usize, usize)>.
    pub fn initialize<R: RngCore>(&self, rng: &mut R,
                                  depth: usize, stack: &mut Vec<(String, usize, usize)>) {
        let reached_min_depth = depth >= self.min_height;
        let reached_max_depth = depth == self.max_height;
        let is_terminal = rng.gen_range(0..(self.func_set.len() + self.term_set.len())) < self.term_set.len();

        if ( reached_max_depth | is_terminal ) && reached_min_depth {
            let (id, _, arity) = self.term_set.sample(rng);
            stack.push((id, depth, arity));
        } else {
            let (id, _, arity) = self.func_set.sample(rng);
            stack.push((id, depth, arity));
            for _ in 0..arity {
                self.initialize(rng, depth+1, stack);
            }
        }
    }
}

pub struct RampedHalfAndHalf<T> 
where
    T: Clone
{
    min_height: usize,
    max_height: usize,
    func_set: OperatorSampler<T>,
    term_set: OperatorSampler<T>
}

impl<T> RampedHalfAndHalf<T> 
where
    T: PartialEq + Clone
{
    pub fn new(min_height: usize, max_height: usize,
               func_set: OperatorSampler<T>, term_set: OperatorSampler<T>)
        -> Self {
            return Self { min_height, max_height, func_set, term_set };
        }
    /// Ramped Half and Half method for intializing trees
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    /// * 'stack' - vector to store initialized tree
    ///
    /// # Returns
    ///
    /// Initialized Vec<(String, usize, usize)>.
    pub fn initialize<R: RngCore>(&self, rng: &mut R, 
                                  stack: &mut Vec<(String, usize, usize)>) {
        let is_grow = rng.gen_bool(0.5);

        if is_grow { 
            let grow: Grow<T> = Grow::new(self.min_height, self.max_height, self.func_set.clone(), self.term_set.clone());
            grow.initialize(rng, 0, stack) 
        }
        if !is_grow { 
            let full: Full<T> = Full::new(self.max_height, self.func_set.clone(), self.term_set.clone());
            full.initialize(rng, 0, stack) 
        }
    }
}

#[cfg(test)]
mod nonlinear_tests {
    use rand::{
        SeedableRng,
        rngs::StdRng
    };
    use crate::genotype::{
        genotype::operator_set_sampler::OperatorSampler, 
        genotype::operator_set_sampler::OperatorSet,
        init::Full, init::Grow
    };

    use super::RampedHalfAndHalf;

    fn test(v: String) -> String {
        v
    }

    fn create_default_sets() -> (OperatorSampler<String>, OperatorSampler<String>) {
        let func_ids: Vec<String> = vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "sin".to_string()];
        let func_ops: Vec<OperatorSet<String>> = vec![test; func_ids.len()];
        let func_arity: Vec<usize> = func_ids.iter()
            .map(|id| match id.as_str() {
                "+" | "-" | "*" | "/" => 2usize,
                "sin" => 1usize,
                _ => panic!("Undefined behavior!")
            }).collect::<Vec<usize>>();

        let func_uniform: Vec<f64> = vec![1.0 / func_ids.len() as f64; func_ids.len()];
        let func_set: OperatorSampler<String> = OperatorSampler::new(&func_ids, &func_ops, &func_arity, &func_uniform);

        let term_ids: Vec<String> = vec!["x".to_string(), "y".to_string(), "c".to_string()];
        let term_ops: Vec<OperatorSet<String>> = vec![test; term_ids.len()];
        let term_arity: Vec<usize> = term_ids.iter()
            .map(|_| 0usize).collect::<Vec<usize>>();

        let term_uniform: Vec<f64> = vec![1.0 / term_ids.len() as f64; term_ids.len()];
        let term_set: OperatorSampler<String> = OperatorSampler::new(&term_ids, &term_ops, &term_arity, &term_uniform);

        return (func_set, term_set);
    }

    #[test]
    fn init_full_works() {
        let (func_set, term_set) = create_default_sets();

        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);
    
        let init_scheme: Full<String> = Full::new(2, func_set, term_set);
        let mut individual = Vec::new();
        init_scheme.initialize(&mut rng, 0, &mut individual);
        
        println!("{:?}", individual);
        //TODO: Create sensible test case -> Chi square test?
    }

    #[test]
    fn init_grow_works() {
        let (func_set, term_set) = create_default_sets();

        let seed: [u8; 32] = [12; 32];
        let mut rng = StdRng::from_seed(seed);
    
        let init_scheme: Grow<String> = Grow::new(2, 5, func_set, term_set);
        let mut individual = Vec::new();
        init_scheme.initialize(&mut rng, 0, &mut individual);
        
        println!("{:?}", individual);
        //TODO: Create sensible test case -> Chi square test?
    }

    #[test]
    fn init_ramped_works() {
        let (func_set, term_set) = create_default_sets();

        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let init_scheme: RampedHalfAndHalf<String> = RampedHalfAndHalf::new(2, 5, func_set, term_set);
        let mut individual = Vec::new();
        init_scheme.initialize(&mut rng, &mut individual);

        println!("{:?}", individual);
        //TODO: Create sensible test case -> Chi square test?
    }
}

