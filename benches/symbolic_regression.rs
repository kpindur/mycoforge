use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mycoforge::operators::set::OperatorsBuilder;
use rand::Rng;
use std::collections::HashMap;
use std::path::Path;
use rand::{rngs::StdRng, SeedableRng};

use rayon::prelude::*;

use mycoforge::dataset::dataset::Dataset;

use mycoforge::{ea_components};
use mycoforge::optimizers::ga::{EABuilder, EAComponents};
use mycoforge::common::traits::{Evaluator, Individual, Initializer, Mutator, Optimizer, Crossoverer};

use mycoforge::tree::core::{tree::TreeGenotype, individual::TreeIndividual};
use mycoforge::tree::operators::{init::Grow, mutation::SubtreeMutation, crossover::SubtreeCrossover, select::TournamentSelection};
use mycoforge::tree::fitness::evaluate::MeanSquared;

use mycoforge::operators::sampler::OperatorSampler;
use mycoforge::operators::functions::symbolic::{add, sub, mul, div};

fn x(data: &[&[f64]]) -> Vec<f64> { return data[0].to_vec();}

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("symbolic_regression");

    let mut rng = StdRng::seed_from_u64(42);
    
    let operators = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 5.0).expect("Failed to add an operator!")
        .add_operator("-", sub, 2, 1.0 / 5.0).expect("Failed to add an operator!")
        .add_operator("*", mul, 2, 1.0 / 5.0).expect("Failed to add an operator!")
        .add_operator("/", div, 2, 1.0 / 5.0).expect("Failed to add an operator!")
        .add_operator("x", x, 0, 1.0 / 5.0).expect("Failed to add an operator!")
        .build().expect("Failed to build operators!");

    let sampler = operators.sampler();
    let pool_size = 100;

    let depths = vec![(1, 2), (2, 4), (4, 6), (6, 8), (8, 10), (10, 12), (12, 14), (14, 16), (16, 18), (18, 20)];

    for (min_depth, max_depth) in depths.clone() {
        group.bench_with_input(
            BenchmarkId::new("grow", format!("d{}_{}", min_depth, max_depth)),
            &(min_depth, max_depth),
            |b, &(min, max)| {
                let init_scheme = Grow::new(min, max);
                b.iter(|| init_scheme.initialize(&mut rng, sampler));
            }
        );
    }

    for (min_depth, max_depth) in depths.clone() {
        let init_scheme = Grow::new(min_depth, max_depth);
        let mutation_scheme = SubtreeMutation::new(1.0);
        let trees = (0..pool_size).map(|_| init_scheme.initialize(&mut rng, sampler)).collect::<Vec<TreeGenotype>>();
        group.bench_function(format!("mutation/d{}_{}", min_depth, max_depth),
            |b| b.iter(|| {
                let idx = rng.gen_range(0..trees.len());
                mutation_scheme.variate(&mut rng, &trees[idx], sampler);
            })
        );
    }

    for (min_depth, max_depth) in depths.clone() {
        let init_scheme = Grow::new(min_depth, max_depth);
        let crossover_scheme = SubtreeCrossover::new(1.0);
        let trees = (0..pool_size).map(|_| init_scheme.initialize(&mut rng, sampler)).collect::<Vec<TreeGenotype>>();
        group.bench_function(format!("crossover/d{}_{}", min_depth, max_depth),
            |b| b.iter(|| {
                let idx1 = rng.gen_range(0..trees.len());
                let idx2 = rng.gen_range(0..trees.len());
                crossover_scheme.variate(&mut rng, &trees[idx1], &trees[idx2], sampler);
            })
        );
    }
    let feature_names = ["x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let no_dims = 2;
    let xs: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let ys = xs.iter().map(|&x| x.powi(2) + x ).collect::<Vec<f64>>();
    let test_data = vec![xs, ys];

    let data = Dataset::new(feature_names, no_dims, test_data.clone(), test_data);
    let map = operators.create_map();

    for (min_depth, max_depth) in depths {
        let init_scheme = Grow::new(min_depth, max_depth);
        let evaluator = MeanSquared::new();

        let trees = (0..pool_size/2).map(|_| init_scheme.initialize(&mut rng, sampler)).collect::<Vec<TreeGenotype>>();
        let trees = trees.iter().cycle().take(pool_size).cloned().collect::<Vec<TreeGenotype>>();

        group.bench_function(format!("evaluation/d{}_{}", min_depth, max_depth),
            |b| b.iter(|| {
                for tree in &trees {
                    evaluator.evaluate(tree, &data, &map);
                }
            }) 
        );

        group.bench_function(format!("evaluation_hashed/d{}_{}", min_depth, max_depth),
            |b| b.iter(|| {
                let mut cache: HashMap<TreeGenotype, f64> = HashMap::new();
                for tree in &trees {
                    if let Some(value) = cache.get(tree) { continue; }
                    cache.insert(tree.clone(), evaluator.evaluate(tree, &data, &map));
                }
            })
        );

        group.bench_function(format!("evaluation_parallel/d{}_{}", min_depth, max_depth),
            |b| b.iter(|| {
                trees.par_iter().for_each(|tree| { evaluator.evaluate(tree, &data, &map); } );
            }) 
        );
    }

    group.bench_function("f1(x)=x^2+x", |b| {
        b.iter(|| {
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
                operators: operators,
                config: {
                    init: Grow::new(2, 4),
                    mutation: SubtreeMutation::new(0.1),
                    crossover: SubtreeCrossover::new(0.9),
                    evaluation: MeanSquared::new(),
                    selection: TournamentSelection::new(5)
                }
            };
            let feature_names = ["x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
            let no_dims = 2;
            let xs: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
            let ys = xs.iter().map(|&x| x.powi(2) + x ).collect::<Vec<f64>>();
            let test_data = vec![xs, ys];

            let data = Dataset::new(feature_names, no_dims, test_data.clone(), test_data);

            let mut rng = StdRng::seed_from_u64(42);
            
            let population_size = 500;
            let initial_population = ea.init_population(&mut rng, population_size);
            let mut fitnesses = initial_population.iter().map(|ind| ea.evaluator().evaluate(ind, &data, ea.map())).collect::<Vec<f64>>();

            let mut population = TreeIndividual::from_vecs(&initial_population, &fitnesses);
            for i in 0..100 {
                let next_population = ea.optimize(&mut rng, &population);
                fitnesses = next_population.iter().map(|ind| ea.evaluator().evaluate(ind, &data, ea.map())).collect::<Vec<f64>>();
                population = TreeIndividual::from_vecs(&next_population, &fitnesses);
            }
        });
    });
    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .output_directory(Path::new("benches_results"))
        .sample_size(100)
        .significance_level(0.1)
        .noise_threshold(0.05);
    targets = benchmark
}

criterion_main!(benches);
