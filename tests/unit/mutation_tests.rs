use rand::rngs::StdRng;
use rand::SeedableRng;

use rstest::{fixture, rstest};

use mycoforge::common::traits::{Initializer, Mutator};

use mycoforge::operators::sampler::OperatorSampler;

use mycoforge::tree::core::tree::TreeGenotype;

use mycoforge::tree::operators::init::Grow;
use mycoforge::tree::operators::mutation::{SizeFairMutation, SubtreeMutation};

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
        assert!(valid_tree(&mutant));
    }
}

#[rstest]
fn test_size_fair_mutation(sample_sampler: OperatorSampler) {
    let mut rng = StdRng::seed_from_u64(42);
    
    let test_cases = vec![
        (false, "static"),
        (true, "dynamic")
    ];

    for (dynamic_limit, case_name) in test_cases {
        let init_scheme = Grow::new(2, 3);
        let tree = init_scheme.initialize(&mut rng, &sample_sampler);
        let original_size = tree.arena().len();

        let mutator = SizeFairMutation::new(1.0, dynamic_limit).expect("Failed to create mutation scheme!");
        let mutant = mutator.variate(&mut rng, &tree, &sample_sampler);

        assert_ne!(tree.arena(), mutant.arena(),
            "{}: Tree enchanged", case_name
        );
        assert!(!mutant.children().is_empty(), 
            "{}: Empty children", case_name
        );
        assert!(valid_tree(&mutant), 
            "{}: Invalid mutant", case_name
        );

        let mutant_size = mutant.arena().len();
        let min_size = (original_size as f64 / 2.0).floor() as usize;
        let max_size = (original_size as f64 * 1.5).ceil() as usize;

        assert!(mutant_size >= min_size,
            "{}: Tree too small! Expected ({}..{}), found {}", 
            case_name, min_size, max_size, mutant_size
        );
        assert!(mutant_size <= max_size,
            "{}: Tree too large! Expected ({}..{}), found {}", 
            case_name, min_size, max_size, mutant_size
        );
    }
}
