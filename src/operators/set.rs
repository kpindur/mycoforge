//! Operator sets and builder for managing GP operators.
//!
//! This module provides structures for:
//! - Managing sets of operators with their arities and weights
//! - Building operator sets with validation
//! - Sampling operators based on weights

use rand::Rng;
use std::collections::HashMap;
use std::fmt;

use crate::operators::sampler::{OperatorSampler, Sampler};
use crate::common::types::VectorFunction;

/// Represents different types of nodes in a GP tree
pub enum NodeType {
    /// Function with name and arity
    Function(String, usize),
    /// Named variable (input feature)
    Variable(String),
    /// Constant numeric value
    Constant(f64),
    /// Generator function for ephemeral random constants
    EphemeralGenerator(Box<dyn Fn() -> f64>)
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Function(name, arity) => write!(f, "Function({}, {})", name, arity),
            NodeType::Variable(name) => write!(f, "Variable({})", name),
            NodeType::Constant(value) => write!(f, "Constant({})", value),
            NodeType::EphemeralGenerator(_) => write!(f, "EphemeralGenerator(<function>)"),
        }
    }
}

impl Clone for NodeType {
    fn clone(&self) -> Self {
        match self {
            NodeType::Function(name, arity) => NodeType::Function(name.clone(), *arity),
            NodeType::Variable(name) => NodeType::Variable(name.clone()),
            NodeType::Constant(value) => NodeType::Constant(*value),
            // For ephemeral constants, we create a new EphemeralGenerator that
            // returns the same value each time - effectively "freezing" the value
            NodeType::EphemeralGenerator(generator) => {
                let value = generator();
                NodeType::Constant(value)
            }
        }
    }
}

impl NodeType {
    /// Returns the arity of the node (how many child nodes it requires)
    pub fn arity(&self) -> usize {
        match self {
            NodeType::Function(_, arity) => *arity,
            // Terminals have arity 0
            NodeType::Variable(_) | NodeType::Constant(_) | NodeType::EphemeralGenerator(_) => 0,
        }
    }

    /// Returns the name of the node (for functions and variables)
    pub fn name(&self) -> String {
        match self {
            NodeType::Function(name, _) => name.clone(),
            NodeType::Variable(name) => name.clone(),
            NodeType::Constant(value) => format!("{}", value),
            NodeType::EphemeralGenerator(_) => "ephemeral".to_string(),
        }
    }
}

/// Interface for operator sets with sampling capability
pub trait OperatorSet {
    /// Returns operator by name.
    fn get_operator(&self, name: &str) -> Option<&Functor>;
    /// Samples random operator based on weights.
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize);
}

/// Container for all node types with sampling functionality.
///
/// # Fields
/// * `functions: HashMap<String, Functor>` - Map of function names to functors
/// * `terminals: Vec<NodeType>` - List of terminal nodes (variables, constants)
/// * `function_sampler: OperatorSampler` - Sampler for function selection
/// * `terminal_sampler: OperatorSampler` - Sampler for terminal selection
pub struct Operators {
    functions: HashMap<String, Functor>,
    terminals: Vec<NodeType>,
    function_sampler: OperatorSampler,
    terminal_sampler: OperatorSampler,
}

impl Operators {
    pub fn new(
        functions: HashMap<String, Functor>, 
        terminals: Vec<NodeType>,
        function_sampler: OperatorSampler,
        terminal_sampler: OperatorSampler
    ) -> Self { 
        return Self { 
            functions, 
            terminals, 
            function_sampler, 
            terminal_sampler 
        }; 
    }
    
    pub fn functions(&self) -> &HashMap<String, Functor> { return &self.functions; }
    pub fn terminals(&self) -> &Vec<NodeType> { return &self.terminals; }
    pub fn function_sampler(&self) -> &OperatorSampler { return &self.function_sampler; }
    pub fn terminal_sampler(&self) -> &OperatorSampler { return &self.terminal_sampler; }

    pub fn functions_mut(&mut self) -> &mut HashMap<String, Functor> { return &mut self.functions; }
    pub fn terminals_mut(&mut self) -> &mut Vec<NodeType> { return &mut self.terminals; }
    pub fn function_sampler_mut(&mut self) -> &mut OperatorSampler { return &mut self.function_sampler; }
    pub fn terminal_sampler_mut(&mut self) -> &mut OperatorSampler { return &mut self.terminal_sampler; }
    
    /// Creates map of operators with their arities and functions. Required for tree evaluations.
    pub fn create_map(&self) -> HashMap<String, (usize, VectorFunction)> {
        let mut map = HashMap::new();
        for (key, value) in &self.functions {
            map.insert(key.clone(), (value.arity(), *value.func()));
        }
        return map;
    }
    
    /// Sample a function node
    pub fn sample_function<R: Rng>(&self, rng: &mut R) -> NodeType {
        let (name, arity) = self.function_sampler.sample(rng);
        return NodeType::Function(name, arity);
    }
    
    /// Sample a terminal node (variable, constant, or ephemeral)
    pub fn sample_terminal<R: Rng>(&self, rng: &mut R) -> NodeType {
        let idx = self.terminal_sampler.sample_index(rng);
        return self.terminals[idx].clone();
    }
    
    /// Sample any node based on a boolean flag
    pub fn sample_node<R: Rng>(&self, rng: &mut R, function_only: bool) -> NodeType {
        if function_only || (rng.random::<f64>() < 0.7) { // Customize probability as needed
            self.sample_function(rng)
        } else {
            self.sample_terminal(rng)
        }
    }

    /// Returns a combined sampler that includes both functions and terminals
    pub fn sampler(&self) -> OperatorSampler {
        // Create a new sampler that combines both function and terminal samplers
        let ops = self.functions.keys().cloned()
            .chain(self.terminals.iter().map(|t| t.name()))
            .collect::<Vec<String>>();
        let arities = self.functions.values()
            .map(|f| f.arity())
            .chain(self.terminals.iter().map(|t| t.arity()))
            .collect::<Vec<usize>>();
        let weights = self.function_sampler.weights().iter()
            .chain(self.terminal_sampler.weights().iter())
            .copied()
            .collect::<Vec<f64>>();
        return OperatorSampler::new(ops, arities, weights);
    }
}

impl OperatorSet for Operators {
    fn get_operator(&self, name: &str) -> Option<&Functor> { return self.functions.get(name); }
    fn sample<R: Rng>(&self, rng: &mut R) -> (String, usize) { return self.function_sampler.sample(rng); }
}

/// Wrapper for operator function with metadata.
///
/// # Fields
/// * `func: VectorFunction` - [`function`][`crate::common::types::VectorFunction`] implementing
/// the operator
/// * `arity: usize` - number of arguments operator takes
/// * `weight: f64` - sampling weight for operator
#[derive(Clone)]
pub struct Functor {
    func: VectorFunction,
    arity: usize,
    weight: f64
}

impl Functor {
    pub fn new(func: VectorFunction, arity: usize, weight: f64) -> Self { return Self { func, arity, weight }; }

    pub fn arity(&self) -> usize { return self.arity; }
    pub fn weight(&self) -> f64 { return self.weight; }
    pub fn func(&self) -> &VectorFunction { return &self.func; }
}