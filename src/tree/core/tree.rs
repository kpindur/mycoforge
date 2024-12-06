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

impl Default for TreeGenotype {
    fn default() -> Self { return Self { arena: Vec::new(), children: HashMap::new() }; }
}
