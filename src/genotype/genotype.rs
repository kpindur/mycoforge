use crate::genotype::traits::*;

pub mod linear_structure {
    use rand::RngCore;
    use crate::genotype::traits::Genotype;

    /// Basic implementation of a linear structure
    #[derive(Debug, Clone)]
    pub struct LinearGenotype<T> {
        seq: Vec<T>,
    }
    
    impl<R, T> Genotype<R, T> for LinearGenotype<T> 
    where
        R: RngCore,
        T: Clone
    {
        fn initialize(rng: &mut R, init_scheme: &impl super::Initialization<R, T>) -> Self {
            let initialized = init_scheme.initialize(rng);
            return Self { seq: initialized };
        }

        fn mutate(&self, rng: &mut R, mutation_scheme: &impl super::Mutation<R, T>) -> Self {
            let mutant = mutation_scheme.mutate(rng, &self.seq);
            return Self { seq: mutant };
        }

        fn crossover(&self, rng: &mut R, other: &Self, crossover_scheme: &impl super::Crossover<R, T>) -> Vec<Self> {
            let children = crossover_scheme.crossover(rng, (&self.seq, &other.seq));
            return children.iter().map(|child| Self { seq: child.to_vec() }).collect::<Vec<Self>>();
        }
    }

    #[cfg(test)]
    mod linear_tests {
        use super::*;
        use crate::genotype::{crossover::linear_structure::OnePointCrossover, init::linear_structure::InitUniform, mutation::linear_structure::UniformBinaryMutation, traits::Initialization};

        use rand::prelude::*;

        #[test]
        fn init_works() {
            let seed: [u8; 32] = [0; 32];
            let mut rng = StdRng::from_seed(seed);

            let pop_size = 100;
            let mut population: Vec<LinearGenotype<bool>> = Vec::new();

            let init_scheme: InitUniform<bool> = InitUniform::new(10);

            for _ in 0..pop_size {
                population.push(LinearGenotype::initialize(&mut rng, &init_scheme));
            }

            assert_eq!(pop_size, population.len(), "Error: population size is wrong!")
        }

        #[test]
        fn mutate_works() {
            let seed: [u8; 32] = [0; 32];
            let mut rng = StdRng::from_seed(seed);
            
            let init_scheme: InitUniform<bool> = InitUniform::new(10);
            let individual: LinearGenotype<bool> = LinearGenotype::initialize(&mut rng, &init_scheme);
            
            let mutation_scheme: UniformBinaryMutation = UniformBinaryMutation::new(1.0);
            let mutant: LinearGenotype<bool> = individual.mutate(&mut rng, &mutation_scheme);
            assert_ne!(individual.seq, mutant.seq,
            "Error: Mutant is exactly the same!");
        }

        #[test]
        fn crossover_works() {
            let seed: [u8; 32] = [2; 32]; // NOTE: Success depends on the seed
            let mut rng = StdRng::from_seed(seed);
            
            let init_scheme: InitUniform<bool> = InitUniform::new(10);
            let parents: Vec<LinearGenotype<bool>> = vec![LinearGenotype::initialize(&mut rng, &init_scheme), LinearGenotype::initialize(&mut rng, &init_scheme)];
            assert_ne!(parents[0].seq, parents[1].seq);
            
            let crossover_scheme: OnePointCrossover = OnePointCrossover::new(1.0);
            let children: Vec<LinearGenotype<bool>> = parents[0].crossover(&mut rng, &parents[1], &crossover_scheme);

            for i in 0..parents.len() {
                for j in 0..children.len() {
                    assert_ne!(parents[i].seq, children[j].seq, 
                            "Error: Parent {} is the same as Child {}!", i, j);
                }
            }
        }
    }
}

pub mod operator_set_sampler {
    use rand::{
        RngCore,
        distributions::{Distribution, WeightedIndex}
    };

    pub type OperatorSet<T> = fn(T) -> T;

    pub struct OperatorSampler<T> {
        ids: Vec<String>,
        ops: Vec<OperatorSet<T>>,
        distribution: WeightedIndex<f64>
    }

    impl<T> OperatorSampler<T> {
        pub fn new(ids: &[String], ops: &[OperatorSet<T>], probs: &[f64]) -> Self {
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
        
        pub fn sample<R: RngCore>(&self, rng: &mut R) -> (String, OperatorSet<T>) {
            let id: usize = self.distribution.sample(rng);

            return (self.ids[id].clone(), self.ops[id].clone());
        }
        
    }

    #[cfg(test)]
    mod distribution_tests {
        use super::*;

        use rand::{
            rngs::StdRng, SeedableRng
        };
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
}

pub mod nonlinear_structure {
    use std::collections::HashMap;

    use super::{operator_set_sampler::{OperatorSampler, OperatorSet}, Genotype};
    use rand::RngCore;

    pub struct Node<T> 
    where
        T: PartialEq
    {
        idx: String,
        val: T,
        parent: Option<usize>
    }

    impl<T> Node<T>
    where
        T: PartialEq + Default 
    {
        fn new(idx: String, val: T, parent: Option<usize>) -> Self {
            return Self { idx, val, parent };
        }

        fn sample_from<R>(rng: &mut R, op_sampler: OperatorSampler<T>) -> Self
        where
            R: RngCore 
        {
            let (sampled_id, _): (String, OperatorSet<T>) = op_sampler.sample(rng);
            return Self { idx: sampled_id, val: T::default(), parent: None };
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
            return Self { arena: Vec::new(), children: HashMap::new()}
        }
    }

    impl<R, T> Genotype<R, T> for TreeGenotype<T>
    where R: RngCore,
          T: PartialEq + Default
    {
        fn initialize(rng: &mut R, init_scheme: &impl super::Initialization<R, T>) -> Self {
            unimplemented!();
        }

        fn mutate(&self, rng: &mut R, mutation_scheme: &impl super::Mutation<R, T>) -> Self {
            unimplemented!();
        }

        fn crossover(&self, rng: &mut R, other: &Self, crossover_scheme: &impl super::Crossover<R, T>) -> Vec<Self> {
            unimplemented!();
        }
    }
    
    #[cfg(test)]
    mod nonlinear_tests {
        #[test]
        fn initialization_works() {
            unimplemented!();
        }
        #[test]
        fn mutation_works() {
            unimplemented!();
        }
        #[test]
        fn crossover_works() {
            unimplemented!();
        }
    }
}
////    pub fn init<T, R>(
////        rng: &mut R,
////        max_depth: usize,
////        func_set: &OperatorSet<T>,
////        term_set: &OperatorSet<T>,
////        method: Method,
////    ) -> SyntaxTree<T>
////    where
////        T: PartialEq + Default,
////        R: Rng,
////    {
////        let mut stack: Vec<usize> = Vec::new();
////        let mut arena: Vec<Node<T>> = Vec::new();
////        let mut children: Vec<usize> = Vec::new();
////
////        let func_size: usize = func_set.len();
////        let term_size: usize = term_set.len();
////
////        let is_terminal: bool = rng.gen_range(0..(term_size + func_size)) < term_size;
////
////        if max_depth == 0 || (method == Method::Grow && is_terminal) {
////            let val: T = Default::default();
////
////            match get_random_operator(rng, term_set) {
////                Some((label, _)) => arena.push(Node::new(label, val)),
////                None => panic!("Something went wrong"),
////            }
////        } else {
////            let val: T = Default::default();
////
////            match get_random_operator(rng, func_set) {
////                Some((label, arity)) => {
////                    arena.push(Node::new(label, val));
////
////                    let parent: usize = arena.len() - 1;
////                    let child_left: usize = arena.len();
////                    let mut child_right: usize = 0;
////
////                    for _ in 0..arity {
////                        child_right = arena.len();
////                    }
////                }
////                None => panic!("Something went wrong"),
////            }
////        }
////
////        let mut tree: SyntaxTree<T> = SyntaxTree::new();
////
////        tree
////    }
