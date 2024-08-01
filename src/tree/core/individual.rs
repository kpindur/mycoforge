use std::collections::HashMap;
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
}

