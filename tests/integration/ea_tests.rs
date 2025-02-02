use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashSet;
use std::error::Error;

use mycoforge::operators::set::{Operators, OperatorsBuilder};
use mycoforge::common::traits::{Optimizer, Evaluator, Individual};

use mycoforge::tree::{
    core::{individual::TreeIndividual,  tree::TreeGenotype}, 
    fitness::evaluate::MeanSquared, 
    operators::{crossover::SubtreeCrossover, init::Grow, mutation::SubtreeMutation, select::TournamentSelection}
};
use mycoforge::dataset::core::Dataset;

use mycoforge::optimizers::ga::{EABuilder, EAComponents};
use mycoforge::operators::functions::symbolic::*;

use mycoforge::ea_components;

fn is_valid_tree(tree: &TreeGenotype) -> bool {
    let num_edges = tree.arena().len() - 1;
    let test_num_edges: usize = tree.children().values().map(|v| v.len()).sum();
    return num_edges == test_num_edges;
}

fn x(args:&[&[f64]]) -> Vec<f64> {
    return args[0].to_vec();
}

#[fixture]
fn sample_operators() -> Operators {
    let operators = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 5.0).expect("Failed to add operator!")
        .add_operator("-", sub, 2, 1.0 / 5.0).expect("Failed to add operator!")
        .add_operator("*", mul, 2, 1.0 / 5.0).expect("Failed to add operator!")
        .add_operator("/", div, 2, 1.0 / 5.0).expect("Failed to add operator!")
        .add_operator("x", x, 0, 1.0 / 5.0).expect("Failed to add operator!")
        .build().expect("Failed to build operators!");

    return operators;
}

#[rstest]
fn test_builder_works(sample_operators: Operators) -> Result<(), Box<dyn Error>> {
    struct Components;
    impl EAComponents<TreeGenotype> for Components {
        type I = TreeIndividual<TreeGenotype>;
        type Init = Grow;
        type Mut = SubtreeMutation;
        type Cross = SubtreeCrossover;
        type Eval = MeanSquared;
        type Sel = TournamentSelection;
    }

    let sampler = sample_operators.sampler().clone();
    let map = sample_operators.create_map();

    let init_scheme = Grow::new(2, 4);
    let mutation_scheme = SubtreeMutation::new(1.0, (1, 2));
    let crossover_scheme = SubtreeCrossover::new(1.0);
    let evaluation_scheme = MeanSquared::new();
    let selection_scheme = TournamentSelection::new(5);

    let _ = EABuilder::<Components, TreeGenotype>::new()
        .set_initializer(init_scheme)
        .set_mutator(mutation_scheme?)
        .set_crossoverer(crossover_scheme?)
        .set_evaluator(evaluation_scheme)
        .set_selector(selection_scheme)
        .set_sampler(sampler)
        .set_map(map)
        .build()
        .expect("Failed to build EA!");

    return Ok(());
}

#[rstest]
fn test_macro_works(sample_operators: Operators) {
    let _ = ea_components! {
        genotype: TreeGenotype,
        individual: TreeIndividual<TreeGenotype>,
        components: {
            init: Grow,
            mutation: SubtreeMutation,
            crossover: SubtreeCrossover,
            evaluation: MeanSquared,
            selection: TournamentSelection
        },
        operators: sample_operators,
        config: {
            init: Grow::new(2, 4),
            mutation: SubtreeMutation::default(),
            crossover: SubtreeCrossover::new(0.9).expect("Failed to create SubtreeCrossover!"),
            evaluation: MeanSquared::new(),
            selection: TournamentSelection::new(5)
        }
    };
}

use rstest::{fixture, rstest};

#[fixture]
fn sample_dataset() -> Dataset {
    let feature_names = ["x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let target_name = "y".to_string();

    let xs: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let ys = xs.iter().map(|&v| v.powi(2) + v ).collect::<Vec<f64>>();

    let features = vec![xs.clone()];
    let target = ys.clone();

    let data = Dataset::new(feature_names, target_name, features, target);
    
    return data;
}

#[rstest]
fn test_optimize_works(sample_operators: Operators, sample_dataset: Dataset) {
    let ea = ea_components! {
        genotype: TreeGenotype,
        individual: TreeIndividual<TreeGenotype>,
        components: {
            init: Grow,
            mutation: SubtreeMutation,
            crossover: SubtreeCrossover,
            evaluation: MeanSquared,
            selection: TournamentSelection
        },
        operators: sample_operators,
        config: {
            init: Grow::new(2, 4),
            mutation: SubtreeMutation::new(0.1, (1, 2)).expect("Failed to create mutation scheme!"),
            crossover: SubtreeCrossover::new(0.9).expect("Failed to create SubtreeCrossover!"),
            evaluation: MeanSquared::new(),
            selection: TournamentSelection::new(7)
        }
    };

    let mut rng = StdRng::seed_from_u64(42);
    
    let population_size = 500;
    let initial_population = ea.init_population(&mut rng, population_size);
    let fitnesses = initial_population.iter()
        .map(|ind| ea.evaluator().evaluate(ind, &sample_dataset, ea.map()))
        .collect::<Vec<f64>>();

    let population = TreeIndividual::from_vecs(&initial_population, &fitnesses);

    assert_eq!(population.len(), population_size);
    assert!(population.iter().all(|ind| is_valid_tree(ind.genotype())));
    
    let max_generations = 10;
    let mut current_population = population.clone();

    let initial_avg_fitness = current_population.iter()
        .map(|ind| ind.phenotype()).sum::<f64>() / population_size as f64;
    let mut final_avg_fitness = f64::MAX;

    for _ in 0..max_generations {
        let next_population = ea.optimize(&mut rng, &current_population);

        assert_eq!(next_population.len(), population_size,
            "Population_size has changed! Expected {} found {}", population_size, next_population.len()
        );
        assert!(next_population.iter().all(is_valid_tree),
            "Next population contains invalid trees!"
        );

        let fitnesses = next_population.iter()
            .map(|ind| ea.evaluator().evaluate(ind, &sample_dataset, ea.map()))
            .collect::<Vec<f64>>();
        final_avg_fitness = fitnesses.iter().sum::<f64>() / population_size as f64;

        let next_population = TreeIndividual::from_vecs(&next_population, &fitnesses);
        
        let _ = next_population.iter()
            .map(|ind| ind.genotype())
            .map(|tree| tree.arena())
            .collect::<HashSet<_>>()
            .len();

        current_population.clone_from(&next_population);
    }
    assert!(final_avg_fitness <= initial_avg_fitness,
        "Population severely degraded after {} generations: {} -> {}", max_generations, initial_avg_fitness, final_avg_fitness
    );
    let improvement = (initial_avg_fitness - final_avg_fitness) / initial_avg_fitness;
    assert!(improvement > 0.1, 
        "Insufficient improvement: {:.2}%", improvement * 100.0,
    );
}
