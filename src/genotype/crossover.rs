use std::marker::PhantomData;
use rand::{
    Rng, RngCore
};

/// One point crossover
pub struct OnePointCrossover<T> {
    probability: f64,
    _marker: PhantomData<T>
}

impl<T> OnePointCrossover<T> 
where
    T: Clone
{
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn crossover<R: RngCore>(&self, rng: &mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>> {
        if rng.gen::<f64>() > self.probability { return vec![parents.0.to_vec(), parents.1.to_vec()]; }

        let (left, right) = (parents.0.to_vec(), parents.1.to_vec());
        let xo_point = rng.gen_range(1..left.len()-1);

        let mut new_left = left[0..xo_point].to_vec();
        new_left.extend_from_slice(&right[xo_point..]);
        let mut new_right = right[0..xo_point].to_vec();
        new_right.extend_from_slice(&left[xo_point..]);

        return vec![new_left, new_right];
    }
}

/// Uniform crossover for trees works (in the common region) like uniform crossover in GAs. That
/// is, the offsrping are created by visiting the nodes in the common region and flipping a coing
/// at each locus to decide whether the corresponding offspring node should be picked from the
/// first or the second parent. If a node to be inherited belongs to the base of the common region
/// and is a function, then the subtree rooted there is inherited as well. With this form of
/// crossover, there can be a greater mixing of the code near the root than with other operators.
pub struct UniformCrossover<T> {
    probability: f64,
    _marker: PhantomData<T>
} 

impl<T> UniformCrossover<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }
}

/// In context-preserving crossover, the crossover points are constrained to have the same
/// coordinates, like in one-point crossover. Note that the crossover points are not limited to the
/// common region.
pub struct ContextPreservingCrossover<T> {
    probability: f64,
    _marker: PhantomData<T>
}

impl<T> ContextPreservingCrossover<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn crossover<R: RngCore>(&self, rng: &mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>> {
        todo!();
    }
}

/// In size-fair crossover the first crossover point is selected randomly, as with standard
/// crossover. Then the size of the subtree to be removed from the first parent is calculated. This
/// is used to constrain the choice of the second crossover point so as to guarantee that the
/// subtree excised from the second parent will not be "unfairly" big.
pub struct SizeFairCrossover<T> {
    probability: f64,
    _marker: PhantomData<T>
}

impl<T> SizeFairCrossover<T> {
    pub fn new(probability: f64) -> Self {
        return Self { probability, _marker: PhantomData };
    }

    pub fn crossover<R: RngCore>(&self, rng: &mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>> {
        todo!();
    }
}

/// NOTE: Other crossovers? c.f. Harries and Smith (1997)

#[cfg(test)]
mod linear_tests {
    use super::OnePointCrossover;
    use rand::{
        rngs::StdRng, SeedableRng
    };
    use crate::genotype::init::InitUniform;

    // TODO: Create a better test for => test distributions of parents and children and
    // compare?
    #[test]
    fn one_point_crossover_works() {
        let seed: [u8; 32] = [2; 32]; // NOTE: Success depends on the seed => Check TODO
        let mut rng = StdRng::from_seed(seed);

        let n = 10;
        let init_scheme = InitUniform::new(n);

        let parents: Vec<Vec<bool>> = vec![init_scheme.initialize(&mut rng), init_scheme.initialize(&mut rng)];
        assert_ne!(parents[0], parents[1]);

        let crossover_rate = 1.0;
        let crossover_scheme = OnePointCrossover::new(crossover_rate);
        let children: Vec<Vec<bool>> = crossover_scheme.crossover(&mut rng, (&parents[0], &parents[1]));

        for i in 0..parents.len() {
            for j in 0..children.len() {
                assert_ne!(parents[i], children[j], 
                        "Error: Parent {} is the same as Child {}!", i, j);
            }
        }
    }
}
