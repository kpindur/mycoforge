use rand::{
    prelude::*,
    RngCore,
    rngs::StdRng,
    SeedableRng,
    distributions::Standard,
};

pub trait Genotype<R>
where
    R: RngCore,
    Self: Sized
{
    fn initialize(rng: &mut R, size: usize) -> Self;
    fn mutate(&self, rng: &mut R, prob: f64) -> Self;
    fn crossover(&self, other: &Self, rng: &mut R, prob: f64) -> Vec<Self>;
}

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
        let mut seq = vec![T::default(); size];
        for i in 0..seq.len() {
            seq[i] = rng.gen::<T>();
        }

        return Self{
            seq
        };
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

//pub struct TreeGenotype {
//
//}
//
//impl<R> Genotype<R> for TreeGenotype 
//where
//    R: RngCore
//{
//    fn initialize(rng: &mut R, size: usize) -> Self {
//        todo!()
//    }
//    
//    fn mutate(&self, rng: &mut R) -> Self {
//        todo!()
//    }
//
//    fn crossover(&self, other: &Self, rng: &mut R) -> Self {
//        todo!()
//    }
//}

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
        // NOTE: Wrong way to test, as there is a chance (although very small) for the individual
        // to not mutate, i.e. individual = mutant
            //  NOTE: Just make the mutation chance 100%?
        assert!(!individual.seq.iter().zip(mutant.seq.iter()).all(|(i, m)| i == m),
        "Error: Mutant is exactly the same!");
    }

    #[test]
    fn crossover_works() {
        let seed: [u8; 32] = [2; 32];
        let mut rng = StdRng::from_seed(seed);

        let parents: Vec<LinearGenotype<bool>> = vec![LinearGenotype::initialize(&mut rng, 10), LinearGenotype::initialize(&mut rng, 10)];
        // NOTE: There should be a smarter way to achieve this
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
