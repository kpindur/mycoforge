use rand::rngs::StdRng;
use rand::SeedableRng;

use rstest::{fixture, rstest};

use mycoforge::common::traits::Initializer;

use mycoforge::tree::core::tree::TreeGenotype;
use mycoforge::operators::sampler::OperatorSampler;

use mycoforge::tree::operators::init::Grow;

fn valid_tree(tree: &TreeGenotype) -> bool {
    let mut result: usize = 0;
    for value in tree.children().values() {
        result += value.len();
    }

    if (result + 1) != tree.arena().len() {
        return false;
    }
    return true;
}

#[fixture]
fn sample_sampler() -> OperatorSampler {
    let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
    let arity = vec![2, 2, 1, 0, 0, 0];
    let weights = vec![1.0 / 6.0; 6];

    let sampler = OperatorSampler::new(operators, arity, weights);

    return sampler;
}

fn grow_test_cases() -> Vec<(u32, u32)> {
    let cases = vec![
        (0, 1),
        (1, 2),
        (2, 3),

        (3, 5),
        (4, 6),
        (4, 7),

        (2, 8),
        (5, 10),
        (2, 12)
    ];
    return cases;
}

#[rstest]
fn test_intializer_grow(sample_sampler: OperatorSampler) {
    let mut rng = StdRng::seed_from_u64(42);
    
    for case in grow_test_cases() {
        let (size_min, size_max) = ((case.0+1) as usize, 2usize.pow(case.1+1)-1);

        let init_scheme = Grow::new(case.0 as usize, case.1 as usize);
        let tree = init_scheme.initialize(&mut rng, &sample_sampler);

        assert!(valid_tree(&tree));
        assert!(tree.arena().len() >= size_min && tree.arena().len() <= size_max, 
            "Wrong tree size for case: ({}, {})! Expected: {} < n < {}. Found: {}", 
            case.0, case.1,
            size_min, size_max, 
            tree.arena().len()
        );
    }
}
