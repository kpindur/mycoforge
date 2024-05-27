use crate::genotype::traits::*;

use std::collections::HashMap;

use rand::{
    prelude::*, 
    distributions::{Standard, WeightedIndex},
    RngCore
};

/// Simple implementation of a linear (sequential) genotype
#[derive(Debug, Clone)]
pub struct LinearGenotype<T> {
    seq: Vec<T>
}

impl<R, T> Genotype<R> for LinearGenotype<T>
where
    R: RngCore,
    T: Default + Clone,
    Standard: Distribution<T>
{
    fn initialize(rng: &mut R, size: usize) -> Self {
        let seq: Vec<T> = (0..size).map(|_| rng.gen::<T>()).collect();

        return Self{ seq };
    }
    
    fn mutate(&self, rng: &mut R, prob: f64) -> Self {
        let mut mutant = self.seq.clone();
        for i in 0..mutant.len() {
            let chance: f64 = rng.gen::<f64>();
            if chance < prob { mutant[i] = rng.gen::<T>(); }
        }
        return Self {
            seq: mutant
        };
    }

    fn crossover(&self, other: &Self, rng: &mut R, prob: f64) -> Vec<Self> {
        if rng.gen::<f64>() > prob { return vec![self.clone(), other.clone()]; }

        let (left, right) = (self.seq.to_vec(), other.seq.to_vec());
        let xo_point = rng.gen_range(1..left.len()-1);

        let mut new_left = left[0..xo_point].to_vec();
        new_left.extend_from_slice(&right[xo_point..]);
        let mut new_right = right[0..xo_point].to_vec();
        new_right.extend_from_slice(&left[xo_point..]);

        return vec![Self{ seq: new_left }, Self { seq: new_right }];
    }
}

#[cfg(test)]
mod linear_tests {
    use super::*;

    #[test]
    fn init_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let pop_size = 100;
        let mut population: Vec<LinearGenotype<bool>> = Vec::new();

        for _ in 0..pop_size {
            population.push(LinearGenotype::initialize(&mut rng, 10));
        }

        assert_eq!(pop_size, population.len(), "Error: population size is wrong!")
    }

    #[test]
    fn mutate_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let individual: LinearGenotype<bool> = LinearGenotype::initialize(&mut rng, 10);

        let mutant: LinearGenotype<bool> = individual.mutate(&mut rng, 1.0);
        assert_ne!(individual.seq, mutant.seq,
        "Error: Mutant is exactly the same!");
    }

    #[test]
    fn crossover_works() {
        let seed: [u8; 32] = [2; 32]; // NOTE: Success depends on the seed
        let mut rng = StdRng::from_seed(seed);

        let parents: Vec<LinearGenotype<bool>> = vec![LinearGenotype::initialize(&mut rng, 10), LinearGenotype::initialize(&mut rng, 10)];
        assert_ne!(parents[0].seq, parents[1].seq);

        let children: Vec<LinearGenotype<bool>> = parents[0].crossover(&parents[1], &mut rng, 1.0);

        for i in 0..parents.len() {
            for j in 0..children.len() {
                assert_ne!(parents[i].seq, children[j].seq, 
                        "Error: Parent {} is the same as Child {}!", i, j);
            }
        }
    }
}

type OperatorSet<T> = fn(T) -> T;

pub struct OperatorSampler<T> {
    ids: Vec<String>,
    ops: Vec<OperatorSet<T>>,
    distribution: WeightedIndex<f64>
}

impl<T> OperatorSampler<T> {
    fn new(ids: &[String], ops: &[OperatorSet<T>], probs: &[f64]) -> Self {
        let lengths_match = ids.len() == ops.len() && ids.len() == probs.len();
        assert!(lengths_match, "Error: Lengths do not match!");
        
        let is_distribution = probs.iter().sum::<f64>() == 1.0;
        assert!(is_distribution, "Error: Probability distribution does not sum to 1.0! Sum: {}", probs.iter().sum::<f64>());

        return Self { 
            ids: ids.to_vec(), 
            ops: ops.to_vec(), 
            distribution: WeightedIndex::new(probs).unwrap()
        };
    }
    
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, OperatorSet<T>) {
        let id: usize = self.distribution.sample(rng);

        return (self.ids[id].clone(), self.ops[id].clone());
    }
    
}

#[cfg(test)]
mod distribution_tests {
    use super::*;
    use std::collections::HashMap;

    fn test(v: String) -> String {
        v
    }
    
    fn chi_square_test(observed: &[String], expected: &HashMap<String, f64>) -> f64 {
        let mut chi_square: f64 = 0.0;
        let mut count: HashMap<String, usize> = HashMap::new();
        
        for id in observed {
            *count.entry(id.clone()).or_insert(0) += 1; 
        }
        
        for key in count.keys() {
            let obs = count.get(key).unwrap();
            let exp = expected.get(key).unwrap() * observed.len() as f64;

            chi_square += (*obs as f64 - exp).powf(2.0) / exp;
        }

        return chi_square;
    }

    #[test]
    fn uniform_sample_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let ids: Vec<String> = vec!["id0".to_string(), "id1".to_string(), "id2".to_string(), "id3".to_string(), "id4".to_string()];
        let ops: Vec<OperatorSet<String>> = vec![test; ids.len()];

        let uniform: Vec<f64> = vec![1.0 / ids.len() as f64; ids.len()];
        let temp: OperatorSampler<String> = OperatorSampler::new(&ids, &ops, &uniform);
        
        let n: usize = 1000;
        let samples: Vec<String> = (0..n).map(|_| temp.sample(&mut rng).0).collect();

        let expected: Vec<(String, f64)> = ids.iter().zip(uniform.iter()).map(|(i, p)| (i.clone(), p.clone())).collect();
        let expected: HashMap<String, f64> = HashMap::from_iter(expected);
        let chi_square = chi_square_test(&samples, &expected);
        
        //NOTE: 9.488 is a critical value for a = 0.05 and df = 4 (ids.len() - 1)
        assert!(chi_square < 9.488, "Error: Chi Square Test failed! {} > {}", chi_square, 9.488);    
    }

    #[test]
    fn custom_sample_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let ids: Vec<String> = vec!["id0".to_string(), "id1".to_string(), "id2".to_string(), "id3".to_string(), "id4".to_string()];
        let ops: Vec<OperatorSet<String>> = vec![test; ids.len()];

        let custom: Vec<f64> = vec![0.25, 0.25, 0.1, 0.1, 0.3];
        let temp: OperatorSampler<String> = OperatorSampler::new(&ids, &ops, &custom);
        
        let n: usize = 1000;
        let samples: Vec<String> = (0..n).map(|_| temp.sample(&mut rng).0).collect();

        let expected: Vec<(String, f64)> = ids.iter().zip(custom.iter()).map(|(i, p)| (i.clone(), p.clone())).collect();
        let expected: HashMap<String, f64> = HashMap::from_iter(expected);
        let chi_square = chi_square_test(&samples, &expected);
        
        //NOTE: 9.488 is a critical value for a = 0.05 and df = 4 (ids.len() - 1)
        // -> possibly change to use chi distribution?
        assert!(chi_square < 9.488, "Error: Chi Square Test failed! {} > {}", chi_square, 9.488);    
    }
}

pub struct Node<T>
where
    T: PartialEq,
{
    idx: String,
    val: T,
    parent: Option<usize>,
}

impl<T> Node<T> 
where
    T: PartialEq + Default
{
    fn new(idx: String, val: T, parent: Option<usize>) -> Self {
        return Self {
            idx, val, parent
        };
    }

    fn sample_from<R: Rng>(rng: &mut R, op_sampler: OperatorSampler<T>) -> Self {
        let (sampled_idx, _): (String, OperatorSet<T>) = op_sampler.sample(rng);
        return Self {
            idx: sampled_idx, val: T::default(), parent: None
        };
    }
}

pub struct TreeGenotype<T> 
where
    T: PartialEq + Default
{
    arena: Vec<Node<T>>,
    children: HashMap<usize, Vec<usize>>
}

impl<T> TreeGenotype<T> 
where
    T: PartialEq + Default
{
    fn new() -> Self {
        return Self { 
            arena: Vec::new(),
            children: HashMap::new()
        };
    }
}

impl<R, T> Genotype<R> for TreeGenotype<T>
where
    R: RngCore,
    T: PartialEq + Default
{
    fn initialize(rng: &mut R, size: usize) -> Self {
        todo!()
    }
//    pub fn init<T, R>(
//        rng: &mut R,
//        max_depth: usize,
//        func_set: &OperatorSet<T>,
//        term_set: &OperatorSet<T>,
//        method: Method,
//    ) -> SyntaxTree<T>
//    where
//        T: PartialEq + Default,
//        R: Rng,
//    {
//        let mut stack: Vec<usize> = Vec::new();
//        let mut arena: Vec<Node<T>> = Vec::new();
//        let mut children: Vec<usize> = Vec::new();
//
//        let func_size: usize = func_set.len();
//        let term_size: usize = term_set.len();
//
//        let is_terminal: bool = rng.gen_range(0..(term_size + func_size)) < term_size;
//
//        if max_depth == 0 || (method == Method::Grow && is_terminal) {
//            let val: T = Default::default();
//
//            match get_random_operator(rng, term_set) {
//                Some((label, _)) => arena.push(Node::new(label, val)),
//                None => panic!("Something went wrong"),
//            }
//        } else {
//            let val: T = Default::default();
//
//            match get_random_operator(rng, func_set) {
//                Some((label, arity)) => {
//                    arena.push(Node::new(label, val));
//
//                    let parent: usize = arena.len() - 1;
//                    let child_left: usize = arena.len();
//                    let mut child_right: usize = 0;
//
//                    for _ in 0..arity {
//                        child_right = arena.len();
//                    }
//                }
//                None => panic!("Something went wrong"),
//            }
//        }
//
//        let mut tree: SyntaxTree<T> = SyntaxTree::new();
//
//        tree
//    }
    
    fn mutate(&self, rng: &mut R, prob: f64) -> Self {
        todo!()
    }

    fn crossover(&self, other: &Self, rng: &mut R, prob: f64) -> Vec<Self> {
        todo!()
    }
}

#[cfg(test)]
mod nonlinear_tests {
    #[test]
    fn init_works() {
        unimplemented!()
    }

    #[test]
    fn mutate_works() {
        unimplemented!()
    }

    #[test]
    fn crossover_works() {
        unimplemented!()
    }
}
