use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use crate::common::traits::Genotype;
use crate::operators::sampler::OperatorSampler;

#[derive(Clone)]
pub struct TreeGenotype {
    arena: Vec<String>,
    children: HashMap<usize, Vec<usize>>,
}

impl Genotype for TreeGenotype {}

impl TreeGenotype {
    pub fn new(arena: Vec<String>, children: HashMap<usize, Vec<usize>>) -> Self { return Self { arena, children }; }
    pub fn with_arena(arena: Vec<String>) -> Self { return Self { arena, children: HashMap::new() }; }

    pub fn arena(&self) -> &Vec<String> { return &self.arena; }
    pub fn arena_mut(&mut self) -> &mut Vec<String> { return &mut self.arena; }
    pub fn children(&self) -> &HashMap<usize, Vec<usize>> { return &self.children; }
    pub fn children_mut(&mut self) -> &mut HashMap<usize, Vec<usize>> { return &mut self.children; }

    pub fn subtree(&self, root: usize) -> usize {
        let mut stack = vec![root];
        let mut last_visited = root;
        
        while let Some(index) = stack.pop() {
            if index > last_visited { last_visited = index; }
            if let Some(children) = self.children.get(&index) {
                for child in children { stack.push(*child); }
            }
        }

        return last_visited;
    }

    pub fn construct_children(&self, sampler: &OperatorSampler) -> HashMap<usize, Vec<usize>> {
        let mut children = HashMap::new();
        let operators = sampler.operators();
        let arities = sampler.arities();

        let mut stack = vec![0]; // Stack of nodes to generate children for
        let mut current = 0;

        while let Some(parent) = stack.pop() {
            if parent != current {
                children.entry(parent)
                    .and_modify(|vec: &mut Vec<usize>| vec.push(current))
                    .or_insert(vec![current]);
            }

            if let Some(op_idx) = operators.iter().position(|op| *op == self.arena()[current]) {
                let arity = arities[op_idx];
                if arity > 0 { 
                    for _ in 0..arity {
                        stack.push(current);
                    }
                }
            }
            current += 1;
        }
        return children;
    }

    fn fmt_node(&self, f: &mut Formatter<'_>, node_index: usize, prefix: &str, child_prefix: &str) -> Result {
        writeln!(f, "{}{}", prefix, self.arena[node_index])?;

        if let Some(children) = self.children.get(&node_index) {
            let child_count = children.len();

            for (i, &child_index) in children.iter().enumerate() {
                let is_last = i == child_count - 1;
                let new_prefix = if is_last {
                    format!("{}└── ", child_prefix)
                } else {
                    format!("{}├── ", child_prefix)
                };
                let new_child_prefix = if is_last {
                    format!("{}    ", child_prefix)
                } else {
                    format!("{}│   ", child_prefix)
                };

                self.fmt_node(f, child_index, &new_prefix, &new_child_prefix)?;
            }
        }

        return Ok(());
    }
}

impl Display for TreeGenotype {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.arena.is_empty() {
            return Ok(());
        }

        self.fmt_node(f, 0, "", "")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let mut tree = TreeGenotype {
            arena: Vec::new(),
            children: HashMap::new(),
        };
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
        
        assert_eq!(tree.arena, arena);
        assert_eq!(tree.children, children);
    }

    #[test]
    fn test_subtree() {
        let mut tree = TreeGenotype {
            arena: vec![
                "+".to_string(),
                "*".to_string(),
                "2".to_string(),
                "x".to_string(),
                "-1".to_string(),
            ],
            children: HashMap::new(),
        };
        tree.children.insert(0, vec![1, 4]);
        tree.children.insert(1, vec![2, 3]);
        
        assert_eq!(tree.subtree(0), 4);
        assert_eq!(tree.subtree(1), 3);
        assert_eq!(tree.subtree(2), 2);
        assert_eq!(tree.subtree(3), 3);
        assert_eq!(tree.subtree(4), 4);
    }

    #[test]
    fn test_construct_children() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);
        
        let arena: Vec<String> = ["+", "*", "2", "x", "-1"].iter().map(|w| w.to_string()).collect();
        let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
        children.insert(0, vec![1, 4]);
        children.insert(1, vec![2, 3]);
        
        let tree = TreeGenotype::with_arena(arena.clone());

        let mut test_tree = TreeGenotype { arena, children: HashMap::new() };
        *test_tree.children_mut() = test_tree.construct_children(&sampler);

        assert_eq!(tree.arena, test_tree.arena);
        assert!(!test_tree.children().is_empty());
        assert_ne!(tree.children(), test_tree.children());
    }

    #[test]
    fn test_construct_children2() {
        let operators: Vec<String> = ["+", "-", "sin", "x", "y", "z"].iter().map(|&w| w.to_string()).collect();
        let arity = vec![2, 2, 1, 0, 0, 0];
        let weights = vec![1.0 / 6.0; 6];

        let sampler = OperatorSampler::new(operators, arity, weights);
        
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
        
        let tree = TreeGenotype::new(arena.clone(), children.clone());

        let mut test_tree = TreeGenotype::with_arena(arena);
        *test_tree.children_mut() = test_tree.construct_children(&sampler);
        
        println!("{:?}", test_tree.arena());
        println!("{:?}", test_tree.children());

        assert_eq!(tree.arena, test_tree.arena);
        assert!(!test_tree.children().is_empty());
        assert_eq!(tree.children(), test_tree.children());
    }

    #[test]
    fn test_tree_display() {
        let tree = TreeGenotype::new(Vec::new(), HashMap::new());

        println!("{}", tree);

        let mut tree = TreeGenotype {
            arena: vec![
                "+".to_string(),
                "*".to_string(),
                "2".to_string(),
                "x".to_string(),
                "-1".to_string(),
            ],
            children: HashMap::new(),
        };
        tree.children.insert(0, vec![1, 4]);
        tree.children.insert(1, vec![2, 3]);

        let expected_output = "\
+
├── *
│   ├── 2
│   └── x
└── -1
";
        println!("{}", tree);
        assert_eq!(format!("{}", tree), expected_output);
    }
}

