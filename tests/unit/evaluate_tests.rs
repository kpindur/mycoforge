use std::collections::HashMap;
use std::error::Error;
use rstest::{fixture, rstest};

use mycoforge::common::traits::Evaluator;

use mycoforge::tree::core::tree::TreeGenotype;

use mycoforge::dataset::dataset::Dataset;
use mycoforge::tree::fitness::evaluate::MeanSquared;

use mycoforge::operators::set::{OperatorsBuilder, Operators};
use mycoforge::operators::functions::symbolic::{add, sub, mul, sin};

#[fixture]
fn sample_function_set() -> Result<Operators, Box<dyn Error>> {
    let sample_operators = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 5.0)?
        .add_operator("-", sub, 2, 1.0 / 5.0)?
        .add_operator("*", mul, 2, 1.0 / 5.0)?
        .add_operator("sin", sin, 1, 1.0 / 5.0)?
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
    let feature_names = ["x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let no_dims = 2;
    let xs: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let ys = xs.iter().map(|&v| v.powi(2) + v ).collect::<Vec<f64>>();
    let test_data = vec![xs, ys];
    
    let data = Dataset::new(feature_names, no_dims, test_data.clone(), test_data);

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

            let tree = TreeGenotype::new(arena.clone(), children.clone());
            tree
        }, sample_dataset(), 3.85)
    ];
}

fn x(args:&[&[f64]]) -> Vec<f64> {
    return args[0].to_vec();
}

#[rstest]
fn test_mse(sample_function_set: Result<Operators, Box<dyn Error>>, test_cases: Vec<(TreeGenotype, Dataset, f64)>) {
    let map: HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)> = sample_function_set.expect("Failed building sample_function_set").create_map();

    let metric = MeanSquared::new();
    let epsilon = 1e-5;

    for (tree, dataset, expected) in test_cases {
        let result = metric.evaluate(&tree, &dataset, &map);
        assert!((expected - result).abs() < epsilon, 
            "Result differs from expected value! {} != {}", expected, result
        );
    }
}
