use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::collections::HashMap;

use rstest::{fixture, rstest};
use serde::Deserialize;

use mycoforge::common::types::VectorFunction;
use mycoforge::common::traits::Evaluator;

use mycoforge::tree::core::tree::TreeGenotype;

use mycoforge::dataset::core::Dataset;
use mycoforge::tree::fitness::evaluate::MeanSquared;

use mycoforge::operators::set::{OperatorsBuilder, Operators};
use mycoforge::operators::functions::symbolic::{add, sub, mul, div};

fn x(args:&[&[f64]]) -> Vec<f64> { return args[0].to_vec(); }

#[fixture]
fn sample_function_set() -> Result<Operators, Box<dyn Error>> {
    let sample_operators = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 5.0)?
        .add_operator("-", sub, 2, 1.0 / 5.0)?
        .add_operator("*", mul, 2, 1.0 / 5.0)?
        .add_operator("/", div, 2, 1.0 / 5.0)?
        .add_operator("x", x, 0, 1.0 / 5.0)?
        .build()?;
    
    return Ok(sample_operators);
}

#[fixture]
fn sample_tree() -> TreeGenotype {
    let arena: Vec<String> = ["+", "*", "x", "x", "x"].iter().map(|w| w.to_string()).collect();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    children.insert(0, vec![1, 4]);
    children.insert(1, vec![2, 3]);

    let tree = TreeGenotype::new(arena.clone(), children.clone());

    return tree;
}

#[fixture]
fn sample_dataset() -> Dataset {
    let data = Dataset::from_csv(&mut rand::thread_rng(), "tests/fixtures/test_f1.csv", 0.0)
        .expect("Failed to load dataset");

    assert_eq!(data.test_data().len(), data.train_data().len(),
        "Test and train data should be of the same length! {} ? {}", 
        data.test_data().len(), data.train_data().len()
    );
    assert_eq!(vec!["x".to_string(), "y".to_string()], *data.feature_names(),
        "Feature names do not match! Expected {:?}, found {:?}", vec!["x", "y"], data.feature_names()
    );

    return data;
}

#[fixture]
fn test_cases() -> Vec<(TreeGenotype, Dataset, f64)> {
    return vec![
        (sample_tree(), sample_dataset(), 0.0),
        ({
            let arena = ["*", "x", "x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
            let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
            children.insert(0, vec![1, 2]);

            TreeGenotype::new(arena.clone(), children.clone())
        }, sample_dataset(), 850.1683501683499)
    ];
}

#[derive(Debug, Clone, Deserialize)]
struct TestIndividual {
    arena: Vec<String>,
    children: HashMap<String, Vec<i32>>,
    fitness: f64
}

#[fixture]
fn deap_test_cases() -> (Vec<(TreeGenotype, f64)>, Dataset) {
    let mut rng = rand::thread_rng();

    let filename = "tests/fixtures/all_populations.json";
    let file = File::open(filename)
        .unwrap_or_else(|e| panic!("failed to open file {}: {}", filename, e));
    let reader = BufReader::new(file);

    let filename = "tests/fixtures/polynomial_dataset.csv";
    let dataset = Dataset::from_csv(&mut rng, filename, 0.0)
        .unwrap_or_else(|e| panic!("failed to load dataset from file {}: {}", filename, e));

    let individuals: Vec<Vec<TestIndividual>> = serde_json::from_reader(reader)
        .unwrap_or_else(|e| panic!("failed to load individuals from file {}: {}", filename, e));
    let individuals = individuals.concat();

    let individuals = individuals.into_iter()
        .map(|ind| {
            // Convert string keys to usize
            let children: HashMap<usize, Vec<usize>> = ind.children
                .into_iter()
                .map(|(k, v)| (
                    k.parse().unwrap(),
                    v.into_iter().map(|x| x as usize).collect()
                ))
                .collect();
            
            (TreeGenotype::new(ind.arena, children), ind.fitness)
        })
        .collect();
    return (individuals, dataset);
}

#[rstest]
fn test_deap_evaluation(
    sample_function_set: Result<Operators, Box<dyn Error>>, 
    deap_test_cases: (Vec<(TreeGenotype, f64)>, Dataset)) 
{
    let map: HashMap<String, (usize, VectorFunction)> = sample_function_set
        .expect("Failed building sample_function_set").create_map();
    let (test_cases, dataset) = deap_test_cases;

    let metric = MeanSquared::new();
    let epsilon = 1e-5;

    for (tree, expected) in test_cases {
        let result = metric.evaluate(&tree, &dataset, &map);
        let result = result / (dataset.train_data()[1].len() as f64);
        assert!((expected - result).abs() < epsilon, 
            "Result differs from expected value! {} != {}", expected, result);
    }
}

#[rstest]
fn test_mse(sample_function_set: Result<Operators, Box<dyn Error>>, test_cases: Vec<(TreeGenotype, Dataset, f64)>) {
    let map: HashMap<String, (usize, VectorFunction)> = sample_function_set.expect("Failed building sample_function_set").create_map();

    let metric = MeanSquared::new();
    let epsilon = 1e-5;

    for (tree, dataset, expected) in test_cases {
        let result = metric.evaluate(&tree, &dataset, &map);
        assert!((expected - result).abs() < epsilon, 
            "Result differs from expected value! {} != {}", expected, result
        );
    }
}
