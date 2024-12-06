use std::collections::HashMap;

use rstest::*;

use mycoforge::operators::sampler::*;
use mycoforge::tree::core::tree::*;

#[fixture]
fn sample_tree_simple() -> TreeGenotype {
    let arena: Vec<String> = ["+", "*", "2", "x", "-1"].iter().map(|w| w.to_string()).collect();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    children.insert(0, vec![1, 4]);
    children.insert(1, vec![2, 3]);
    
    return TreeGenotype::new(arena, children);
}

#[fixture]
fn sample_tree_complex() -> TreeGenotype {
    let arena: Vec<String> = ["-", "-", "-", "y", "sin", "y", "z", "-", "sin", "+", "x", "y", "y"].iter()
        .map(|w| w.to_string()).collect();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    children.insert(0, vec![1, 7]);
    children.insert(1, vec![2, 6]);
    children.insert(2, vec![3, 4]);
    children.insert(4, vec![5]);
    children.insert(7, vec![8, 12]);
    children.insert(8, vec![9]);
    children.insert(9, vec![10, 11]);

    return TreeGenotype::new(arena, children);
}

fn sample_trees() -> impl Iterator<Item = TreeGenotype> {
    return vec![
        sample_tree_simple(), sample_tree_complex()
    ].into_iter();
}

#[fixture]
fn sample_sampler() -> OperatorSampler {
    let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
    let arity = vec![2, 2, 1, 0, 0, 0];
    let weights = vec![1.0 / 6.0; 6];

    let sampler = OperatorSampler::new(operators, arity, weights);

    return sampler;
}

#[test]
fn test_access() {
    let mut tree = TreeGenotype::default();

    let arena: Vec<String> = ["+", "*", "2", "x", "-1"].iter().map(|w| w.to_string()).collect();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    children.insert(0, vec![1, 4]);
    children.insert(1, vec![2, 3]);

    for node in &arena {
        tree.arena_mut().push(node.to_string());
    }

    for (&key, value) in children.iter() {
        tree.children_mut().insert(key, value.clone());
    }
    
    assert_eq!(tree.arena(), &arena);
    assert_eq!(tree.children(), &children);
}

#[rstest]
#[case(0, 4)]
#[case(1, 3)]
#[case(2, 2)]
#[case(3, 3)]
#[case(4, 4)]
fn test_subtree(#[case] root: usize, #[case] expected: usize, sample_tree_simple: TreeGenotype) {
    assert_eq!(sample_tree_simple.subtree(root), expected);
}

#[rstest]
fn test_construct_children(sample_sampler: OperatorSampler) {
    for sample_tree in sample_trees() {
        let mut tree = TreeGenotype::with_arena(sample_tree.arena().clone());
        *tree.children_mut() = tree.construct_children(&sample_sampler);

        assert_eq!(sample_tree.arena(), tree.arena());

        assert!(!tree.children().is_empty());
    }
}

#[rstest]
fn test_tree_display(sample_tree_simple: TreeGenotype) {
    let expected_output = "\
+
├── *
│   ├── 2
│   └── x
└── -1
";
    assert_eq!(format!("{}", sample_tree_simple), expected_output);
}
