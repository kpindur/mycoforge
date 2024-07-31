use std::marker::PhantomData;
use rand::{
    Rng, RngCore
};

use crate::genotype::genotype::operator_set_sampler::OperatorSampler;
use crate::genotype::init::Grow;

use super::genotype::{Node, TreeGenotype};

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

/// Inserts `subtree` into `genotype` between `mutation_point` and `subtree_end`.
///
/// # Arguments
///
/// * 'genotype' - original tree structure
/// * `subtree` - generated tree structure to insert into `genotype`
/// * `mutation_point` - randomly selected root node of a subtree
/// * `subtree_end` - the end index of a tree rooted at mutation_point
///
/// # Returns
///
/// Created TreeGenotype<T> as a tuple.
fn combine<T>(genotype: (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>),
              subtree: (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>),
              mutation_point: usize, subtree_end: usize
             ) -> (Vec<Node<T>>, Vec<usize>, Vec<usize>)
where
    T: PartialEq + Clone + Default
{
    let (arena, depth, arity) = genotype;
    let (o_arena, o_depth, o_arity) = subtree;
    
    let mut mutant = (arena[0..mutation_point].to_vec(), 
                      depth[0..mutation_point].to_vec(), 
                      arity[0..mutation_point].to_vec());
    mutant.0.extend_from_slice(o_arena);
    mutant.1.extend_from_slice(o_depth);
    mutant.2.extend_from_slice(o_arity);
    if subtree_end < arena.len() {
        mutant.0.extend_from_slice(&arena[subtree_end..]);
        mutant.1.extend_from_slice(&depth[subtree_end..]);
        mutant.2.extend_from_slice(&arity[subtree_end..]);
    }

    return mutant;
}

/// Replace the element of `genotype` at `mutation_point` with `node`
///
/// # Arguments
///
/// * `genotype` - original tree structure
/// * `node` - node represented as a tuple
/// * `mutation_point` - randomly selected root node of a subtree
///
/// # Returns
/// Create TreeGenotype<T> as a tuple.
fn replace<T>(genotype: (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>),
              node: (Node<T>, usize, usize),
              mutation_point: usize
             ) -> (Vec<Node<T>>, Vec<usize>, Vec<usize>)
where
    T: PartialEq + Clone + Default
{
    let (mut arena, mut depth, mut arity) = (genotype.0.clone(), genotype.1.clone(), genotype.2.clone());
    
    arena[mutation_point] = node.0;
    depth[mutation_point] = node.1;
    arity[mutation_point] = node.2;

    let mutant = (arena, depth, arity);

    return mutant;
}

/// Calculates depth of the full binary tree from number of nodes.
///
/// # Arguments
///
/// * `no_nodes` - number of nodes of a tree
/// * 'lower' - flag to either floor (for lower limit) or ceil (for upper limit)
///
/// # Returns
///
/// Calculated depth of the full binary tree.
fn to_depth(no_nodes: f64, lower: bool) -> usize {
    let no_nodes = if lower { no_nodes.floor() } else { no_nodes.ceil() };

    return (((no_nodes).ceil() + 1.0).log2() - 1.0) as usize;
}

/// Subtree mutation replaces a randomly selected subtree with another randomly created subtree.
/// Kinnear defined a similar mutation operator, but with a restriction that prevents the offspring
/// from being more than 15% deeper than its parent.
pub struct SubtreeMutation<T> 
where
    T: Clone
{
    probability:    f64,
    size_limit:     Option<f64>,
    func_set:       OperatorSampler<T>,
    term_set:       OperatorSampler<T>,
}

impl<T> SubtreeMutation<T> 
where
    T: PartialEq + Clone + Default
{
    pub fn new(probability: f64, size_limit: Option<f64>, 
               func_set: OperatorSampler<T>, term_set: OperatorSampler<T>
               ) -> Self {
        return Self { probability, size_limit, func_set, term_set };
    }

    /// Mutate tree structure using stored probabilities, limits and function sets.
    /// Selects mutation point uniformly and generates new subtree.
    ///
    /// # Arguments
    ///
    /// * `rng` - mutable random number generator
    /// * 'genotype' - tree genotype structure
    ///
    /// # Returns
    ///
    /// Updated TreeGenotype<T>.
    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: &TreeGenotype<T>) -> TreeGenotype<T> {
        let mutate = rng.gen::<f64>() < self.probability;
        if !mutate { return genotype.clone(); }

        let mutation_point = rng.gen_range(0..genotype.len());
        let subtree_end = genotype.dfs(mutation_point);

        let mut max_height: usize = 2;
        if let Some(max_size) = self.size_limit {
            max_height = to_depth(max_size * genotype.len() as f64, false);
        }

        let init_scheme = Grow::new(0, max_height, self.func_set.clone(), self.term_set.clone());

        let mut new_subtree: Vec<(String, usize, usize)> = Vec::new();
        init_scheme.initialize(rng, 0, &mut new_subtree);
    
        let (mut arena, mut depth, mut arity) = (Vec::new(), Vec::new(), Vec::new());
        for (id, d, a) in new_subtree {
            arena.push(Node::new(id.to_string()));
            depth.push(d+genotype.depth(mutation_point));
            arity.push(a);
        }
        let subtree: (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>) = (&arena, &depth, &arity);
        
        let mutant = combine::<T>(genotype.get_tuple(), subtree, mutation_point, subtree_end);

        return TreeGenotype::from_tuple(mutant);
    }
}

/// Size-fair subtree muttation was proposed in two forms by Langdon. 
/// In both cases, the new random subtree is, on average, the same size as the code it replaces. 
/// The size of the random code is given either by the size of another random subtree in the 
/// program or chosen at random in the range [ l/2, 3l/2 ]. 
/// The first of these methods samples uniformly in the space of possible programs, whereas 
/// the second samples uniformly in the space of program lengths. Experiments suggested that 
/// there was far more bloat with the first mutation operator.
pub struct SizeFairMutation<T> 
where
    T: Clone
{
    probability:    f64,
    size_limit:     bool,
    func_set:       OperatorSampler<T>,
    term_set:       OperatorSampler<T>,
}

impl<T> SizeFairMutation<T> 
where
    T: PartialEq + Clone + Default
{
    pub fn new(probability: f64, size_limit: bool, 
               func_set: OperatorSampler<T>, term_set: OperatorSampler<T>
               ) -> Self {
        return Self { probability, size_limit, func_set, term_set };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: TreeGenotype<T>) -> TreeGenotype<T> {
        let mutate = rng.gen::<f64>() < self.probability;
        if !mutate { return genotype.clone(); }
        // Subtree to substitute
        let mutation_point = rng.gen_range(0..genotype.len());
        let subtree_end = genotype.dfs(mutation_point);
        // Static Limit
        let mut min_height: usize = to_depth(genotype.len() as f64 / 2.0, true);        // NOTE: L/2
        let mut max_height: usize = to_depth(3.0 * genotype.len() as f64 / 2.0, false); // NOTE: 3L/2
        // Dynamic Limit
        if self.size_limit {
            let no_nodes = (subtree_end - mutation_point) as f64;

            min_height = 0;                         // NOTE: L/2
            max_height = to_depth(no_nodes, false); // NOTE: 3L/2
        }
        let init_scheme = Grow::new(min_height, max_height, self.func_set.clone(), self.term_set.clone());

        let mut new_subtree: Vec<(String, usize, usize)> = Vec::new();
        init_scheme.initialize(rng, 0, &mut new_subtree);
    
        let (mut arena, mut depth, mut arity) = (Vec::new(), Vec::new(), Vec::new());
        for (id, d, a) in new_subtree {
            arena.push(Node::new(id.to_string()));
            depth.push(d+genotype.depth(mutation_point));
            arity.push(a);
        }
        let subtree: (&Vec<Node<T>>, &Vec<usize>, &Vec<usize>) = (&arena, &depth, &arity);
        
        let mutant = combine::<T>(genotype.get_tuple(), subtree, mutation_point, subtree_end);

        return TreeGenotype::new(mutant.0, mutant.1, mutant.2);
    }
}

/// Node replacement mutation (also known as point mutation) is similar to bit string mutation in
/// that it randomly changes a point in the individual. In linear GAs the change would be a bit
/// flip. In GP, instead a node in the tree is randomly selected and randomly changed. To ensure
/// the tree remains legal, the replacement node has the same number of arguments as the node it is
/// replacing.
pub struct PointMutation<T> 
where
    T: Clone
{
    probability:    f64,
    func_set:       OperatorSampler<T>,
    term_set:       OperatorSampler<T>,
}

impl<T> PointMutation<T> 
where
    T: PartialEq + Clone + Default
{
    pub fn new(probability: f64, func_set: OperatorSampler<T>, term_set: OperatorSampler<T>) -> Self {
        return Self { probability, func_set, term_set };
    }

    pub fn mutate<R: RngCore>(&self, rng: &mut R, genotype: TreeGenotype<T>) -> TreeGenotype<T> {
        let mutate = rng.gen::<f64>() < self.probability;
        if !mutate { return genotype.clone(); }

        let mutation_point = rng.gen_range(0..genotype.len());
        let (depth, arity): (&usize, &usize) = (genotype.depth(mutation_point), genotype.arity(mutation_point));
        let mut node: Node<T> = if *arity == 0 {
            Node::new(self.term_set.sample(rng).0)
        } else {
            let max_reps = 100;
            let mut new_id: String = String::new();
            for _ in 0..max_reps {
                let (sampled_id, _, sampled_arity) = self.func_set.sample(rng);
                if sampled_arity != *arity { continue; }
                new_id = sampled_id;
                break;
            }
            Node::new(new_id)
        };

        let node: (Node<T>, usize, usize) = (node, *depth,*arity);
        let mutant = replace::<T>(genotype.get_tuple(), node, mutation_point);

        return TreeGenotype::new(mutant.0, mutant.1, mutant.2);
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

    use crate::genotype::init::{Full, Grow, RampedHalfAndHalf};

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

    #[test]
    fn subtree_mutation_works() {
        let seed: [u8; 32] = [0; 32];
        let mut rng = StdRng::from_seed(seed);

        //let init_scheme = 
        unimplemented!()
    }

    #[test]
    fn size_fair_mutation_works() {
        unimplemented!()
    }

    #[test]
    fn point_mutation_works() {
        unimplemented!()
    }
}
