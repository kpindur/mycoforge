#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq)]
pub enum Order {
    Prefix,
    Infix,
    Postfix,
}

pub struct Node<T>
where
    T: PartialEq,
{
    idx: String,
    val: T,
    parent: Option<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(idx: String, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
        }
    }
}

pub struct SyntaxTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
    children: HashMap<usize, Vec<usize>>,
}

impl<T> SyntaxTree<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: Node<T>, children: Option<Vec<usize>>) {
        self.arena.push(node);
        if let Some(children) = children {
            self.children.insert(self.arena.len() - 1, children);
        }
    }

    fn delete(&mut self, idx: usize) {
        self.arena.remove(idx);
        self.children.remove(&idx);
    }

    fn search(&self) {
        todo!();
    }

    pub fn dfs(&self, idx: usize, order: Order) -> Option<Vec<usize>> {
        if self.arena.get(idx).is_none() {
            return None;
        }

        match order {
            Order::Prefix => Some(self.preorder(idx)),
            Order::Postfix => Some(self.postorder(idx)),
            Order::Infix => Some(self.inorder(idx)),
        }
    }

    fn preorder(&self, idx: usize) -> Vec<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut result: Vec<usize> = Vec::new();
        stack.push(idx);

        while let Some(current) = stack.pop() {
            result.push(current);

            if let Some(children) = self.children.get(&current) {
                for &child in children.iter().rev() {
                    stack.push(child);
                }
            }
        }
        result
    }

    fn postorder(&self, idx: usize) -> Vec<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut result: Vec<usize> = Vec::new();
        stack.push(idx);

        while let Some(current) = stack.pop() {
            if current != idx {
                result.push(current);
            }
            if let Some(children) = self.children.get(&current) {
                for &child in children.iter().rev() {
                    stack.push(child)
                }
            }
        }
        result.push(idx);
        result
    }

    fn inorder(&self, idx: usize) -> Vec<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = Vec::new();

        let mut current: usize = idx;
        while !stack.is_empty() || current != usize::MAX {
            while current != usize::MAX {
                if visited.contains(&current) {
                    break;
                }
                stack.push(current);
                current = self
                    .children
                    .get(&current)
                    .and_then(|children| children.first())
                    .copied()
                    .unwrap_or(usize::MAX);
            }

            if let Some(idx) = stack.pop() {
                result.push(idx);
                visited.insert(idx);

                current = self
                    .children
                    .get(&idx)
                    .and_then(|children| children.get(1))
                    .copied()
                    .unwrap_or(usize::MAX);
            }
        }
        result
    }

    pub fn bfs(&self, idx: usize) -> Option<Vec<usize>> {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut result: Vec<usize> = Vec::new();
        queue.push_back(idx);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            result.push(current);

            if let Some(children) = self.children.get(&current) {
                for child in children {
                    queue.push_back(*child);
                }
            }
        }
        Some(result)
    }

    fn root(&self) -> Option<&Node<T>> {
        self.arena.get(0)
    }
    fn parent(&self, idx: usize) -> Option<usize> {
        self.arena.get(idx).unwrap().parent
    }
    fn children(&self, idx: usize) -> &Vec<usize> {
        &self.children.get(&idx).unwrap()
    }

    fn is_leaf(&self, idx: usize) -> bool {
        if self.children.get(&idx).is_some() {
            true
        } else {
            false
        }
    }
    fn height(&self, idx: usize) -> usize {
        todo!()
    }
    fn depth(&self, idx: usize) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }
    fn size(&self) -> usize {
        self.arena.len()
    }
    fn clear(&mut self) {
        self.arena = Vec::new();
        self.children = HashMap::new();
    }
}
