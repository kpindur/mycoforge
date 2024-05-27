
pub mod linear_structure {
    use crate::genotype::traits::Crossover;
    use rand::{
        Rng, RngCore,
        distributions::{Distribution, Standard}, 
    };

    pub struct OnePointCrossover {
        probability: f64,
    }
    
    impl OnePointCrossover {
        pub fn new(probability: f64) -> Self {
            return Self { probability };
        }
    }

    impl<R, T> Crossover<R, T> for OnePointCrossover 
    where
        R: RngCore,
        Standard: Distribution<T>,
        T: Clone
    {
        fn crossover(&self, rng: &mut R, parents: (&Vec<T>, &Vec<T>)) -> Vec<Vec<T>> {
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


    #[cfg(test)]
    mod linear_tests {
        use crate::genotype::traits::Crossover;
        use super::OnePointCrossover;
        use rand::{
            rngs::StdRng, SeedableRng
        };
        use crate::genotype::{init::linear_structure::InitUniform, traits::Initialization};
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
}
