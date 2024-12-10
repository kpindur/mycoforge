use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use rand::{rngs::StdRng, SeedableRng};

use mycoforge::dataset::dataset::Dataset;

use mycoforge::{ea_components};
use mycoforge::optimizers::ga::{EABuilder, EAComponents};
use mycoforge::common::traits::{Individual, Evaluator, Optimizer};

use mycoforge::tree::core::{tree::TreeGenotype, individual::TreeIndividual};
use mycoforge::tree::operators::{init::Grow, mutation::SubtreeMutation, crossover::SubtreeCrossover, select::TournamentSelection};
use mycoforge::tree::fitness::evaluate::MeanSquared;

use mycoforge::operators::{sampler::OperatorSampler, set, functions};
use mycoforge::operators::functions::symbolic::{add, sub, mul, div, sin, cos};

fn x(data: &[&[f64]]) -> Vec<f64> { return data[0].to_vec();}

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("symbolic_regression");

    group.bench_function("f1_simple", |b| {
        b.iter(|| {
            // Create a sensible macro or something to create a koza set
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
                operators: {
                    "+" => (add, 2, 0.2),
                    "-" => (sub, 2, 0.2),
                    "*" => (mul, 2, 0.2),
                    "/" => (div, 2, 0.2),
                    "x" => (x, 0, 0.2)
                },
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
            let ys = xs.iter().map(|&v| v.powi(2) + v ).collect::<Vec<f64>>();
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
            //println!("Best fitness: {}", population.iter().map(|ind| ind.phenotype()).min_by(|&a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap());
        });
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
