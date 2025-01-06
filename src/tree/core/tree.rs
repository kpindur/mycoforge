//! Core tree structure for tree-based Genetic Programming
//!
//! This module provides the [`TreeGenotype`] structure that represents programs as trees using a
//! linear array (arena) in postfix notation with explicit child references.
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::common::traits::Genotype;
use crate::operators::sampler::OperatorSampler;

/// Tree structure for representing programs in Genetic Programming.
/// Uses arena=based representation with hashmap of parent-child relationships.
///
/// # Fields
/// * `arena: Vec<String>` - flat array storing nodes (operators and terminals) in postfix order
/// * `children: HashMap<usize, Vec<usize>>` - maps parent indices to their children indices
#[cfg_attr(feature = "serder", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct TreeGenotype {
    arena: Vec<String>,
    children: HashMap<usize, Vec<usize>>,
}

impl Genotype for TreeGenotype {}

impl TreeGenotype {
    /// Creates new tree with provided arena and children mapping.
    pub fn new(arena: Vec<String>, children: HashMap<usize, Vec<usize>>) -> Self { return Self { arena, children }; }
    /// Creates new tree with provided arena and empty children mapping.
    pub fn with_arena(arena: Vec<String>) -> Self { return Self { arena, children: HashMap::new() }; }

    pub fn arena(&self) -> &Vec<String> { return &self.arena; }
    pub fn arena_mut(&mut self) -> &mut Vec<String> { return &mut self.arena; }
    pub fn children(&self) -> &HashMap<usize, Vec<usize>> { return &self.children; }
    pub fn children_mut(&mut self) -> &mut HashMap<usize, Vec<usize>> { return &mut self.children; }

    /// Returns index of last node in subtree rooted at given index.
    ///
    /// # Arguments
    /// * `root: usize` - index of subtree root
    ///
    /// # Returns
    /// * `usize` - index of last node in subtree
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
    
    /// Constructs children mapping from flat arena representation.
    ///
    /// # Arguments
    /// * `sampler: &OperatorSampler` - provides operator arities and labels for tree construction
    ///
    /// # Returns
    /// * `HashMap<usize, Vec<usiz>>` - mapping of parent indices to children indices
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
        return Ok(());
    }
}

impl Default for TreeGenotype {
    fn default() -> Self { return Self::new(Vec::new(), HashMap::new()); }
}

impl Hash for TreeGenotype {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hashable = self.arena.iter().fold(
            String::new(), |mut hashable, word| {
                hashable.push_str(word);
                hashable
        });
        hashable.hash(state);
    }
}

impl Eq for TreeGenotype {}

impl PartialEq for TreeGenotype {
    fn eq(&self, other: &Self) -> bool {
        return self.arena == other.arena;
    }
}
