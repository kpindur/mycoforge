
pub mod linear_structure {
    use crate::genotype::traits::Mutation;
    use rand::{
        Rng, RngCore
    };

    pub struct UniformBinaryMutation {
        probability: f64,
    }
    
    impl UniformBinaryMutation {
        pub fn new(probability: f64) -> Self {
            return Self { probability };
        }
    }

    impl<R> Mutation<R, bool> for UniformBinaryMutation 
    where
        R: RngCore
    {
        fn mutate(&self, rng: &mut R, genotype: &[bool]) -> Vec<bool> {
            let mut mutant: Vec<bool> = Vec::from(genotype);
            for i in 0..mutant.len() {
                let chance: f64 = rng.gen::<f64>();
                if chance > self.probability { mutant[i] = rng.gen::<bool>(); }
            }
            return mutant;
        }
    }


    #[cfg(test)]
    mod linear_tests {
        use super::*;
        use crate::genotype::{init::linear_structure::InitUniform, traits::Initialization};

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
}
