use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use crate::common::traits::Genotype;

#[derive(Clone)]
pub struct TreeGenotype {
    arena: Vec<String>,
    children: HashMap<usize, Vec<usize>>,
}

impl Genotype for TreeGenotype {}

impl TreeGenotype {
    pub fn new(arena: Vec<String>, children: HashMap<usize, Vec<usize>>) -> Self {
        return Self { arena, children };
    }

    pub fn arena_mut(&mut self) -> &mut Vec<String> { return &mut self.arena; }
    pub fn children_mut(&mut self) -> &mut HashMap<usize, Vec<usize>> { return &mut self.children; }

    pub fn subtree(&self, _root: usize) -> usize {
        // Returns end point
        todo!()
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
    fn test_tree_display() {
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

