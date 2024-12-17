use rstest::{fixture, rstest};

use rand::rngs::StdRng;
use rand::SeedableRng;

use mycoforge::common::traits::{Initializer, Crossoverer};

use mycoforge::tree::core::tree::TreeGenotype;

use mycoforge::operators::sampler::OperatorSampler;

use mycoforge::tree::operators::init::Grow;
use mycoforge::tree::operators::crossover::SubtreeCrossover;

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
fn test_subtree_crossover(sample_sampler: OperatorSampler) {
    let mut rng = StdRng::seed_from_u64(42);

    for case in grow_test_cases() {
        let init_scheme = Grow::new(case.0, case.1);
        let parent1 = init_scheme.initialize(&mut rng, &sample_sampler);
        let parent2 = init_scheme.initialize(&mut rng, &sample_sampler);
 
        for parent in [&parent1, &parent2] {
            assert!(
                valid_tree(parent),
                "Found invalid tree! Found parent {:?} with children {:?}", parent.arena(), parent.children()
            );
        }
        
        let crossover = SubtreeCrossover::new(1.0).expect("Failed to create SubtreeCrossover!");
        let mut children = crossover
            .variate(&mut rng, &parent1, &parent2, &sample_sampler);
        
        for child in &mut children {
            *child.children_mut() = child.construct_children(&sample_sampler);
        }

        for i in 0..children.len() {
            let differs_from_parents = parent1.arena() != children[i].arena() || parent2.arena() != children[i].arena();
            assert!(differs_from_parents,
                "Child should differ at least from one parent! Found: {:?}", children[i].arena()
            );

            if children[0].arena().len() == 1 {
                assert!(
                    children[0].children().is_empty(), 
                    "Children should be empty! Found {:?} for {:?}", children[0].children(), children[0].arena()
                );
            } else {
                assert!(
                    !children[0].children().is_empty(),
                    "Children shoould not be empty! Found {:?} for {:?}", children[0].children(), children[1].arena()
                );
            }

            assert!(valid_tree(&children[i]),
                "Created invalid tree! Found tree {:?} with children {:?}", children[i].arena(), children[i].children()
            );

        }
    }
}
