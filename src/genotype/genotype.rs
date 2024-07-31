use std::collections::{HashMap, HashSet};
use rand::{
    RngCore,
    distributions::Standard,
    prelude::Distribution
};

use crate::genotype::enums::{
    Initialization, Mutation,
    Genotype
};
use crate::genotype::enums::Crossover;

/// Basic implementation of a linear structure
#[derive(Debug, Clone)]
pub struct LinearGenotype<T> {
    seq: Vec<T>,
}

impl<R, T> Genotype<R, T> for LinearGenotype<T> 
where
    R: RngCore,
    T: Clone,
    Standard: Distribution<T>
{
    fn initialize(rng: &mut R, init_scheme: &Initialization<T>) -> Self {
        let initialized = match init_scheme {
            Initialization::Uniform(scheme)             => scheme.initialize(rng),
            Initialization::FromDistribution(scheme)    => scheme.initialize(rng),
            _ => panic!("Something went wrong!")
        };
        return Self { seq: initialized };
    }

    fn mutate(&self, rng: &mut R, mutation_scheme: &Mutation<T>) -> Self {
        todo!()
     //   let mutant = match mutation_scheme {
     //       Mutation::UniformBinary(scheme) => scheme.mutate(rng, self.seq.as_slice()),
     //       _ => panic!("Something went wrong!")
     //   };
     //   return Self { seq: mutant };
    }

    fn crossover(&self, rng: &mut R, other: &Self, crossover_scheme: &impl Crossover<R, T>) -> Vec<Self> {
        todo!()
        //let children = crossover_scheme.crossover(rng, (&self.seq, &other.seq));
        //return children.iter().map(|child| Self { seq: child.to_vec() }).collect::<Vec<Self>>();
    }
}

#[cfg(test)]
mod linear_tests {
    use rand::{
        RngCore, SeedableRng,
        rngs::StdRng,
        distributions::Standard,
        prelude::Distribution
    };

    use crate::genotype::{
        init::InitUniform,
        enums::{Initialization, Genotype},
        genotype::LinearGenotype
//        crossover::linear_structure::OnePointCrossover, 
//        init::linear_structure::InitUniform, 
//        mutation::linear_structure::UniformBinaryMutation
    };

    #[test]
    fn init_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        let pop_size = 100;
        let mut population: Vec<LinearGenotype<bool>> = Vec::new();

        let init_scheme: InitUniform<bool> = InitUniform::new(10);
        let init_scheme: Initialization<bool> = Initialization::Uniform(init_scheme);

        for _ in 0..pop_size {
            population.push(LinearGenotype::initialize(&mut rng, &init_scheme));
        }

        assert_eq!(pop_size, population.len(), "Error: population size is wrong!")
    }

//    #[test]
//    fn mutate_works() {
//        let seed: [u8; 32] = [0; 32];
//        let mut rng = StdRng::from_seed(seed);
//        
//        let init_scheme: InitUniform<bool> = InitUniform::new(10);
//        let individual: LinearGenotype<bool> = LinearGenotype::initialize(&mut rng, &init_scheme);
//        
//        let mutation_scheme: UniformBinaryMutation = UniformBinaryMutation::new(1.0);
//        let mutant: LinearGenotype<bool> = individual.mutate(&mut rng, &mutation_scheme);
//        assert_ne!(individual.seq, mutant.seq,
//        "Error: Mutant is exactly the same!");
//    }
//
//    #[test]
//    fn crossover_works() {
//        let seed: [u8; 32] = [2; 32]; // NOTE: Success depends on the seed
//        let mut rng = StdRng::from_seed(seed);
//        
//        let init_scheme: InitUniform<bool> = InitUniform::new(10);
//        let parents: Vec<LinearGenotype<bool>> = vec![LinearGenotype::initialize(&mut rng, &init_scheme), LinearGenotype::initialize(&mut rng, &init_scheme)];
//        assert_ne!(parents[0].seq, parents[1].seq);
//        
//        let crossover_scheme: OnePointCrossover = OnePointCrossover::new(1.0);
//        let children: Vec<LinearGenotype<bool>> = parents[0].crossover(&mut rng, &parents[1], &crossover_scheme);
//
//        for i in 0..parents.len() {
//            for j in 0..children.len() {
//                assert_ne!(parents[i].seq, children[j].seq, 
//                        "Error: Parent {} is the same as Child {}!", i, j);
//            }
//        }
//    }
}

pub mod operator_set_sampler {
    use rand::{
        RngCore,
        distributions::{Distribution, WeightedIndex}
    };

    pub type OperatorSet<T> = fn(T) -> T;
    
    #[derive(Clone)]
    pub struct OperatorSampler<T> 
    where
        T: Clone
    {
        ids: Vec<String>,
        ops: Vec<OperatorSet<T>>,
        arity: Vec<usize>,
        distribution: WeightedIndex<f64>
    }

    impl<T> OperatorSampler<T> 
    where
        T: Clone
    {
        pub fn new(ids: &[String], ops: &[OperatorSet<T>], arity: &[usize], probs: &[f64]) -> Self {
            let lengths_match = ids.len() == ops.len() && ids.len() == probs.len() && ids.len() == arity.len();
            assert!(lengths_match, "Error: Lengths do not match!");
            
            let is_distribution = probs.iter().sum::<f64>() == 1.0;
            assert!(is_distribution, "Error: Probability distribution does not sum to 1.0! Sum: {}", probs.iter().sum::<f64>());

            return Self { 
                ids: ids.to_vec(), 
                ops: ops.to_vec(), 
                arity: arity.to_vec(),
                distribution: WeightedIndex::new(probs).unwrap()
            };
        }
        pub fn sample<R: RngCore>(&self, rng: &mut R) -> (String, OperatorSet<T>, usize) {
            let id: usize = self.distribution.sample(rng);

            return (self.ids[id].clone(), self.ops[id].clone(), self.arity[id].clone());
        }

        pub fn len(&self) -> usize { return self.ids.len(); }
        
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
            let arity: Vec<usize> = (0..ids.len()).map(|_| 1usize).collect();

            let uniform: Vec<f64> = vec![1.0 / ids.len() as f64; ids.len()];
            let temp: OperatorSampler<String> = OperatorSampler::new(&ids, &ops, &arity, &uniform);
            
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
            let arity: Vec<usize> = (0..ids.len()).map(|_| 1usize).collect();

            let custom: Vec<f64> = vec![0.25, 0.25, 0.1, 0.1, 0.3];
            let temp: OperatorSampler<String> = OperatorSampler::new(&ids, &ops, &arity, &custom);
            
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

use operator_set_sampler::OperatorSampler;

#[derive(Debug, Clone)]
pub struct Node<T> 
where
    T: PartialEq
{
    idx: String,
    val: T,
}

impl<T> Node<T>
where
    T: PartialEq + Default + Clone
{
    pub fn new(idx: String) -> Self {
        return Self { idx, val: T::default() };
    }

    pub fn evaluate<R: RngCore>(rng: &mut R, op_sampler: OperatorSampler<T>) -> Vec<T> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct TreeGenotype<T> 
where
    T: PartialEq + Default + Clone
{
    arena: Vec<Node<T>>,
    depth: Vec<usize>,
    arity: Vec<usize>,
}

impl<T> TreeGenotype<T>
where
    T: PartialEq + Default + Clone
{
    pub fn new(arena: Vec<Node<T>>, depth: Vec<usize>, arity: Vec<usize>) -> Self {
        return Self { arena, depth, arity }
    }
    pub fn from_tuple(individual: (Vec<Node<T>>, Vec<usize>, Vec<usize>)) -> Self {
        return Self { arena: individual.0, depth: individual.1, arity: individual.2 };
    }
    pub fn len(&self) -> usize {
        return self.arena.len();
    }
    pub fn root(&self) -> &Node<T> {
        return self.arena.get(0).expect("Failed to get root!");
    }
    pub fn depth(&self, root: usize) -> &usize {
        return self.depth.get(root).expect("Failed to get depth!");
    }
    pub fn arity(&self, root: usize) -> &usize {
        return self.arity.get(root).expect("Failed to get arity!");
    }
    fn is_leaf(&self, idx: usize) -> bool {
        return match self.arity(idx) {
            0 => true,
            _ => false,
        }
    }
    pub fn get_tuple(&self) -> (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>) {
        return (&self.arena, &self.depth, &self.arity);
    }

    pub fn dfs(&self, root: usize) -> usize {
        if root >= self.arena.len() { return 0; }
        
        let mut start = root;
        let mut end = start;

        for _ in 0..self.arity[start] {
            let child_root = start+1;
            let child_end = self.dfs(child_root);

            end = child_end;
            start = child_end;
        }
        return end;
    }
}

impl<R, T> Genotype<R, T> for TreeGenotype<T> 
where
    R: RngCore,
    T: PartialEq + Default + Clone,
    Standard: Distribution<T>
{
    fn initialize(rng: &mut R, init_scheme: &Initialization<T>) -> Self {
        let mut initialized: Vec<(String, usize, usize)> = Vec::new();
        match init_scheme {
            Initialization::Full(scheme)                => scheme.initialize(rng, 0, &mut initialized),
            Initialization::Grow(scheme)                => scheme.initialize(rng, 0, &mut initialized),
            Initialization::RampedHalfAndHalf(scheme)   => scheme.initialize(rng, &mut initialized),
            _ => panic!("Something went wrong!")
        };
        let (mut arena, mut depth, mut arity) = (Vec::new(), Vec::new(), Vec::new());
        for (id, d, a) in initialized {
            arena.push(Node::new(id.to_string()));
            depth.push(d);
            arity.push(a);
        }

        return Self { arena, depth, arity };
    }

    fn mutate(&self, rng: &mut R, mutation_scheme: &Mutation<T>) -> Self {
        todo!()
     //   let mutant = match mutation_scheme {
     //       Mutation::UniformBinary(scheme) => scheme.mutate(rng, self.seq.as_slice()),
     //       _ => panic!("Something went wrong!")
     //   };
     //   return Self { seq: mutant };
    }

    fn crossover(&self, rng: &mut R, other: &Self, crossover_scheme: &impl Crossover<R, T>) -> Vec<Self> {
        todo!()
        //let children = crossover_scheme.crossover(rng, (&self.seq, &other.seq));
        //return children.iter().map(|child| Self { seq: child.to_vec() }).collect::<Vec<Self>>();
    }
}

#[cfg(test)]
mod nonlinear_tests {
    use super::{Node, TreeGenotype};

    #[test]
    fn dfs_works() {
        let arena: Vec<Node<f64>> = ["+", "*", "x", "y", "log", "x"].iter().map(|c| Node::new(c.to_string())).collect();
        let depth: Vec<usize> = vec![ 0, 1, 2, 2, 1, 2 ];
        let arity: Vec<usize> = vec![ 2, 2, 0, 0, 1, 0 ];

        let tree: TreeGenotype<f64> = TreeGenotype { arena, depth, arity };

        let results: Vec<(usize, usize)> = vec![(0, 5), (1, 3), (2, 2), (3, 3), (4, 5), (5, 5)];
        
        for (i, result) in results.iter().enumerate() {
            let current_res = (i, tree.dfs(i));
            assert_eq!(*result, current_res);
        }

        let arena: Vec<Node<f64>> = ["+", "sin", "*", "*", "2", "pi", "x", "*", "5", "log", "*", "x", "x"].iter().map(|c| Node::new(c.to_string())).collect();
        let depth: Vec<usize> = vec![0, 1, 2, 3, 4, 4, 3, 1, 2, 2, 3, 4, 4];
        let arity: Vec<usize> = vec![2, 1, 2, 2, 0, 0, 0, 2, 0, 1, 2, 0, 0];

        let tree: TreeGenotype<f64> = TreeGenotype { arena, depth, arity };

        let results: Vec<(usize, usize)> = vec![(0, 12), (1, 6), (2, 6), (3, 5), (4, 4), (5, 5), (6, 6), (7, 12), (8, 8), (9, 12), (10, 12), (11, 11), (12, 12)];
        for (i, result) in results.iter().enumerate() {
            let current_res = (i, tree.dfs(i));
            assert_eq!(*result, current_res);
        }
    }
}
