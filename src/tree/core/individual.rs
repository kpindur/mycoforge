use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use crate::common::traits::Genotype;

pub struct TreeGenotype {
    _arena: Vec<String>,
    _children: HashMap<usize, Vec<usize>>,
}

impl TreeGenotype {}

impl Genotype for TreeGenotype {
    fn new() -> Self {
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
}

