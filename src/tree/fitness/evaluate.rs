use std::collections::HashMap;

use crate::common::traits::Data;
use crate::{common::traits::Evaluator, tree::core::individual::TreeGenotype};
use crate::tree::fitness::dataset::Dataset;

//Mean Squared Error (MSE) - most popular
//Root Mean Squared Error (RMSE)
//Mean Absolute Error (MAE)

pub struct MeanSquared {}

impl MeanSquared {
    pub fn new() -> Self { return Self {} }
}

impl Evaluator<TreeGenotype> for MeanSquared {
    type D = Dataset;

    fn evaluate(&self, tree: &TreeGenotype, data: &Self::D, map: HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)>) -> f64 {
        let mut stack: Vec<Vec<f64>> = Vec::new();

        for i in (0..tree.arena().len()).rev() {
            let node = &tree.arena()[i];
            println!("Node {}: {}", i, node);

            if let Some((arity, op)) = map.get(node) {
                match arity {
                    0 => {
                        let operands = data.data_train().1.iter().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                        let result = op(&operands);
                        stack.push(result);
                    },
                    n => {
                        let mut operands = Vec::new();
                        for _ in 0..*n {
                            operands.push(stack.pop().unwrap());
                        }
                        let operands = operands.iter().rev().map(|v| v.as_slice()).collect::<Vec<&[f64]>>();
                        let result = op(&operands);
                        stack.push(result);
                    },
                }
            }
        }
        let (no_dims, operands) = data.data_train();
        let truths = &operands[no_dims-1];
        let result = stack.pop().unwrap().iter()
            .zip(truths.iter())
            .map(|(t,y )| (t - y).powi(2))
            .sum::<f64>();
        return result;
    }
}

#[cfg(test)]
pub mod test {
    use crate::{common::traits::Evaluator, tree::fitness::{dataset::Dataset, evaluate::MeanSquared}};

    #[test]
    fn test_mse() {
        use std::collections::HashMap;
        use crate::tree::core::individual::TreeGenotype;
        use crate::tree::core::sampler::OperatorSampler;

        fn add(args: &[&[f64]]) -> Vec<f64> {
            if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
            return args[0].iter().zip(args[1].iter())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<f64>>();
        }
        fn sub(args: &[&[f64]]) -> Vec<f64> {
            if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
            return args[0].iter().zip(args[1].iter())
                .map(|(&a, &b)| a - b)
                .collect::<Vec<f64>>();
        }
        fn mul(args: &[&[f64]]) -> Vec<f64> {
            if args.len() != 2 || args[0].is_empty() || args[1].is_empty() { return Vec::new(); }
            return args[0].iter().zip(args[1].iter())
                .map(|(&a, &b)| a * b)
                .collect::<Vec<f64>>();
        }
        fn sin(args:&[&[f64]]) -> Vec<f64> {
            if args.len() != 1 || args[0].is_empty() { return Vec::new(); }
            return args[0].iter()
                .map(|&a| a.sin())
                .collect::<Vec<f64>>();
        }
        fn x(args:&[&[f64]]) -> Vec<f64> {
            return args[0].to_vec();
        }

        let functions: Vec<fn(&[&[f64]])-> Vec<f64>> = vec![ add, sub, mul, sin, x ];

        let operators: Vec<String> = ["+", "-", "*","sin", "x"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 2, 1, 0];
        let weights = vec![1.0 / 5.0; 5];

        let sampler = OperatorSampler::new(operators.clone(), arity.clone(), weights);
        
        let arena: Vec<String> = ["+", "*", "x", "x", "x"].iter().map(|w| w.to_string()).collect();
        let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
        children.insert(0, vec![1, 4]);
        children.insert(1, vec![2, 3]);

        let tree = TreeGenotype::new(arena.clone(), children.clone());

        let feature_names = ["x"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
        let no_dims = 2;
        let xs: Vec<f64> = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        let ys = xs.iter().map(|&v| v.powi(2) + v ).collect::<Vec<f64>>();
        let test_data = vec![xs, ys];

        let map: HashMap<String, (usize, fn(&[&[f64]])-> Vec<f64>)> = operators.into_iter()
            .zip(arity.iter().zip(functions.iter())).map(|(la, (&ar, &op))| (la, (ar, op))).collect();
        println!("Map: {:?}", map);

        let data = Dataset::new(feature_names, no_dims, test_data.clone(), test_data);

        let metric = MeanSquared::new();

        let result = metric.evaluate(&tree, &data, map);

        assert_eq!(0.0, result);
    }
}
