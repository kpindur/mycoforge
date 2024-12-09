use std::error::Error;
use rand::{SeedableRng, rngs::StdRng};

use rstest::{fixture, rstest};

use mycoforge::operators::functions::symbolic::{add, sub, mul, sin};

use mycoforge::operators::set::{OperatorsBuilder, Operators};
use mycoforge::operators::sampler::Sampler;

fn x(args:&[&[f64]]) -> Vec<f64> {
    return args[0].to_vec();
}

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

#[rstest]
fn test_update_weights(sample_function_set: Result<Operators, Box<dyn Error>>) {
    let mut function_set = sample_function_set.expect("Failed to build sampler_function_set!");
    let sampler = function_set.sampler_mut();
    let new_weights = vec![2.0, 1.0, 3.0, 4.0, 0.5];
    sampler.update_weights(new_weights.clone());

    assert_eq!(*sampler.weights(), new_weights);
}

#[rstest]
#[case((0, 0, 1))]
#[case((1, 2, 4))]
#[case((1, 1, 1))]
#[case((2, 2, 3))]
fn test_sampler_with_arity(#[case] case: (usize, usize, usize), sample_function_set: Result<Operators, Box<dyn Error>>) {
    let function_set = sample_function_set.expect("Failed to build sample_function_set!");
    let sampler = function_set.sampler();
    let (min_arity, max_arity, answer) = case;

    let sampled_operators = sampler.sampler_with_arity(min_arity, max_arity);

    assert_eq!(answer, sampled_operators.operators().len());
}

#[rstest]
#[case(100)]
#[case(1000)]
fn test_operator_sampler_distribution(#[case] n_samples: usize, sample_function_set: Result<Operators, Box<dyn Error>>) {
    let function_set = sample_function_set.expect("Failed to build sample_function_set!");
    let sampler = function_set.sampler();
    
    let mut rng = StdRng::seed_from_u64(42);
    let mut observed = [0; 5];

    for _ in 0..n_samples {
        let sample = sampler.sample(&mut rng);
        match sample.0.as_str() {
            "+" => observed[0] += 1,
            "-" => observed[1] += 1,
            "*" => observed[2] += 1,
            "sin" => observed[3] += 1,
            "x" => observed[4] += 1,
            _ => panic!("Unexpected sample"),
        }
    }

    let expected = [
        n_samples as f64 * (1.0 / 5.0),
        n_samples as f64 * (1.0 / 5.0),
        n_samples as f64 * (1.0 / 5.0),
        n_samples as f64 * (1.0 / 5.0),
        n_samples as f64 * (1.0 / 5.0),
    ];

    let chi_square: f64 = observed.iter().zip(expected.iter())
        .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
        .sum();
    
    // Degrees of freedom: 3 - 1 = 2
    // For 95% confidence and 2 degrees of freedom, critical value is about 5.991
    assert!(chi_square < 5.991, "Chi-square test failed");
}
