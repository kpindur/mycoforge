use std::marker::PhantomData;

use rand::{
    Rng, RngCore
};

/// Mutation Cookbook
/// 
/// With linear bit string GAs, mutation usually consists of random changes in bit values.
/// In contrast, in GP there are many mutation operators in use. Often multiple types of mutation
/// are beneficially used simultaneously. 
///
/// Implemented mutation operators:
/// * Subtree mutation
/// * Size-fair subtree mutation
/// * Node replacement mutation
/// * Hoist mutation
/// * Shrink mutation
/// * Permutation mutation
/// * Mutating constants at random
/// * Mutating constants systematically


/// Subtree mutation replaces a randomly selected subtree with another randomly created subtree.
/// Kinnear defined a similar mutation operator, but with a restriction that prevents the offspring
/// from being more than 15% deeper than its parent.
pub struct SubtreeMutation<T> {
    probability:    f64,
    size_limit:     Option<usize>,
    _marker:        PhantomData<T>
}

impl<T> SubtreeMutation<T> {
    pub fn new(probability: f64, size_limit: Option<usize>) -> Self {
        return Self { probability, size_limit, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Size-fair subtree muttation was proposed in two forms by Langdon. 
/// In both cases, the new random subtree is, on average, the same size as the code it replaces. 
/// The size of the random code is given either by the size of another random subtree in the 
/// program or chosen at random in the range [ l/2, 3l/2 ]. 
/// The first of these methods samples uniformly in the space of possible programs, whereas 
/// the second samples uniformly in the space of program lengths. Experiments suggested that 
/// there was far more bloat with the first mutation operator.
pub struct SizeFairMutation<T> {
    probability:    f64,
    size_limit:     Option<usize>,
    _marker:        PhantomData<T>
}

impl<T> SizeFairMutation<T> {
    pub fn new(probability: f64, size_limit: Option<usize>) -> Self {
        return Self { probability, size_limit, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Node replacement mutation (also known as point mutation) is similar to bit string mutation in
/// that it randomly changes a point in the individual. In linear GAs the change would be a bit
/// flip. In GP, instead a node in the tree is randomly selected and randomly changed. To ensure
/// the tree remains legal, the replacement node has the same number of arguments as the node it is
/// replacing.
pub struct PointMutation<T> {
    probability:    f64,
    size_limit:     Option<usize>,
    _marker:        PhantomData<T>
}

impl<T> PointMutation<T> {
    pub fn new(probability: f64, size_limit: Option<usize>) -> Self {
        return Self { probability, size_limit, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Hoist mutation creates a new offspring individual which is copy of a randomly chosen subtree of
/// the subtree of the parent. Thus, the offspring will be smaller than the parent and will have a
/// different root node.
pub struct HoistMutation<T> {
    probability:    f64,
    _marker:        PhantomData<T>
}


impl<T> HoistMutation<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Shrink mutation replaces a randomly chosen subtree with a randomly created terminal. This is a
/// special case of subtree mutation where the replacement tree is a terminal. As with hoist
/// mutation, it is motivated by the desire to reduce program size.
pub struct ShrinkMutation<T> {
    probability:    f64,
    _marker:        PhantomData<T>
}

impl<T> ShrinkMutation<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotypes: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Permutation mutation selects a random function node in a tree and then randomly permuting its
/// arguments (subtrees). Koza used permutation in one experiment where it was shown to have little
/// effect. In contrast, Maxwell had more success with a mutation operator called swap, which is
/// simply a permutation mutation restricted to binary non-commutative functions.
pub struct PermutationMutation<T> {
    probability:    f64,
    _marker:        PhantomData<T>
}

impl<T> PermutationMutation<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotypes: &[T]) -> Vec<T> {
        todo!();
    }
}

/// Mutating constants at random:
/// Schoenauer, Sebag, Jouve, Lamy, and Maitournam mutated constants by adding random noise from a
/// Gaussian distribution. Each change to a constant was considered a separate mutation.
pub struct RandomMutation {}

/// Mutating constants systemtically:
/// A variety of potentially expensive optimisation tools have been applied to try and fine-tune an
/// existing an existing program bu finding the "best" value for the constants within it. Indeed
/// STROGANOFF optimises each tree modified by crossover. Clever mechanisms are employed to
/// minimise the computation required.
pub struct SystematicMutation {}

// NOTE: Can be generalised to PointMutation
pub struct UniformBinaryMutation {
    probability: f64,
}

impl UniformBinaryMutation {
    pub fn new(probability: f64) -> Self {
        return Self { probability };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &[bool]) -> Vec<bool> {
        let mut mutant: Vec<bool> = Vec::from(genotype);
        for i in 0..mutant.len() {
            let chance: f64 = rng.gen::<f64>();
            if chance < self.probability { mutant[i] = rng.gen::<bool>(); }
        }
        return mutant;
    }
}

#[cfg(test)]
mod linear_tests {
    use crate::genotype::mutation::UniformBinaryMutation;
    use crate::genotype::init::InitUniform;

    use rand::{
        rngs::StdRng, 
        SeedableRng
    };

    #[test]
    fn uniform_binary_mutation_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);
        
        let n = 10;
        let init_scheme = InitUniform::new(n);

        let individual: Vec<bool> = init_scheme.initialize(&mut rng);
        
        let mutation_rate: f64 = 0.5;
        let mutation_scheme = UniformBinaryMutation::new(mutation_rate);
        let mutant: Vec<bool> = mutation_scheme.mutate(&mut rng, &individual);
        assert_ne!(individual, mutant,
        "Error: Mutant is exactly the same!");
    }
}
