mod syntax_tree;

pub mod initialization {
    #![allow(dead_code)]
    use rand::Rng;
    use std::collections::HashMap;

    use crate::syntax_tree::*;

    #[derive(PartialEq)]
    pub enum Method {
        Full,
        Grow,
    }

    type OperatorSet<T> = HashMap<String, (usize, fn(&Vec<Vec<T>>))>;

    fn get_random_operator<T, R>(rng: &mut R, op_set: &OperatorSet<T>) -> Option<(String, usize)>
    where
        R: Rng,
    {
        let rand: usize = rng.gen_range(0..op_set.len());

        let key = op_set.keys().nth(rand)?;
        let operator = op_set.get(key)?;

        Some((key.to_string(), operator.0))
    }

    pub fn init<T, R>(
        rng: &mut R,
        max_depth: usize,
        func_set: &OperatorSet<T>,
        term_set: &OperatorSet<T>,
        method: Method,
    ) -> SyntaxTree<T>
    where
        T: PartialEq + Default,
        R: Rng,
    {
        let mut stack: Vec<usize> = Vec::new();
        let mut arena: Vec<Node<T>> = Vec::new();
        let mut children: Vec<usize> = Vec::new();

        let func_size: usize = func_set.len();
        let term_size: usize = term_set.len();

        let is_terminal: bool = rng.gen_range(0..(term_size + func_size)) < term_size;

        if max_depth == 0 || (method == Method::Grow && is_terminal) {
            let val: T = Default::default();

            match get_random_operator(rng, term_set) {
                Some((label, _)) => arena.push(Node::new(label, val)),
                None => panic!("Something went wrong"),
            }
        } else {
            let val: T = Default::default();

            match get_random_operator(rng, func_set) {
                Some((label, arity)) => {
                    arena.push(Node::new(label, val));

                    let parent: usize = arena.len() - 1;
                    let child_left: usize = arena.len();
                    let mut child_right: usize = 0;

                    for _ in 0..arity {
                        child_right = arena.len();
                    }
                }
                None => panic!("Something went wrong"),
            }
        }

        let mut tree: SyntaxTree<T> = SyntaxTree::new();

        tree
    }
}
