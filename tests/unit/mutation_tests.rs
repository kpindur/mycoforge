use rand::rngs::StdRng;
use rand::SeedableRng;

use rstest::{fixture, rstest};

use mycoforge::common::traits::{Initializer, Mutator};

use mycoforge::operators::sampler::OperatorSampler;

use mycoforge::tree::core::tree::TreeGenotype;

use mycoforge::tree::operators::init::Grow;
use mycoforge::tree::operators::mutation::SubtreeMutation;

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

fn grow_test_cases() -> Vec<(usize, usize)> {
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
fn test_subtree_mutation(sample_sampler: OperatorSampler) {
    let mut rng = StdRng::seed_from_u64(42);
    for case in grow_test_cases() {
        let init_scheme = Grow::new(case.0, case.1);
        let tree = init_scheme.initialize(&mut rng, &sample_sampler);
        
        let mutator = SubtreeMutation::new(1.0, (1, 2)).expect("Failed to create mutation scheme!");
        let mutant = mutator.variate(&mut rng, &tree, &sample_sampler);

        assert_ne!(tree.arena(), mutant.arena());
        assert!(!mutant.children().is_empty());
        assert!(valid_tree(&tree));
    }
}
