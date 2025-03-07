//! Builder for creating operator sets with all node types.
//!
//! This module provides a builder for creating operator sets that include all types of nodes:
//! - Functions with names and arities
//! - Variables (input features)
//! - Constants (fixed numeric values)

use crate::operators::set::{NodeType, Functor, Operators};
use crate::operators::sampler::OperatorSampler;
use crate::common::types::VectorFunction;

use std::collections::HashMap;

/// Errors that can occur during operator set building.
///
/// # Variants
/// * `IncorrectWeight` - weight not in (0, 1] range
/// * `KeyExists` - operator with given name already exists 
/// * `OperatorsIsEmpty` - no operators added to builder
/// * `WrongWeightSum` - weights don't sum to 1.0
#[derive(Debug)]
pub enum BuilderError {
    IncorrectWeight,
    KeyExists,
    OperatorsIsEmpty,
    WrongWeightSum
}

impl std::error::Error for BuilderError {}
impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectWeight => write!(f, "IncorrectWeight"),
            Self::KeyExists => write!(f, "KeyExists"),
            Self::OperatorsIsEmpty => write!(f, "OperatorsIsEmpty"),
            Self::WrongWeightSum => write!(f, "WrongWeightSum"),
        }
    }
}

/// Builder for creating validated operator sets with all node types.
///
/// # Fields
/// * `functions: HashMap<String, Functor>` - map of functions to be validated
/// * `terminals: Vec<NodeType>` - list of terminal nodes
/// * `function_weights_sum: f64` - running sum of function weights
/// * `terminal_weights_sum: f64` - running sum of terminal weights
pub struct OperatorsBuilder {
    functions: HashMap<String, Functor>,
    terminals: Vec<NodeType>,
    terminal_weights: Vec<f64>,
    function_weights_sum: f64,
    terminal_weights_sum: f64,
}

impl OperatorsBuilder {
    pub fn new(
        functions: HashMap<String, Functor>, 
        terminals: Vec<NodeType>,
        terminal_weights: Vec<f64>,
        function_weights_sum: f64,
        terminal_weights_sum: f64
    ) -> Self {
        return Self { 
            functions, 
            terminals, 
            terminal_weights,
            function_weights_sum, 
            terminal_weights_sum 
        };
    }
    
    /// Adds new function operator to the set.
    ///
    /// # arguments
    /// * `name: &str` - operator name
    /// * `func: VectorFunction` - operator function
    /// * `arity: usize` - number of arguments
    /// * `weight: f64` - sampling weight
    ///
    /// # Returns
    /// * `Result<Self, BuilderError>` - Update builder or [`error`][`BuilderError`]
    pub fn add_function(mut self, name: &str, func: VectorFunction, arity: usize, weight: f64) 
        -> Result<Self, BuilderError> {
            if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
            if self.functions.contains_key(name) { return Err(BuilderError::KeyExists); }
            
            self.functions.insert(name.to_string(), Functor::new(func, arity, weight));
            self.function_weights_sum += weight;

            return Ok(self);
    }
    
    /// Adds variable terminal to the set
    pub fn add_variable(mut self, name: &str, weight: f64) -> Result<Self, BuilderError> {
        if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
        
        self.terminals.push(NodeType::Variable(name.to_string()));
        self.terminal_weights.push(weight);
        self.terminal_weights_sum += weight;
        
        return Ok(self);
    }
    
    /// Adds constant terminal to the set
    pub fn add_constant(mut self, value: f64, weight: f64) -> Result<Self, BuilderError> {
        if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
        
        self.terminals.push(NodeType::Constant(value));
        self.terminal_weights.push(weight);
        self.terminal_weights_sum += weight;
        
        return Ok(self);
    }
    
    /// Adds ephemeral random constant generator to the set
    pub fn add_ephemeral(mut self, generator: Box<dyn Fn() -> f64>, weight: f64) -> Result<Self, BuilderError> {
        if weight <= 0.0 || weight > 1.0 { return Err(BuilderError::IncorrectWeight); }
        
        self.terminals.push(NodeType::EphemeralGenerator(generator));
        self.terminal_weights.push(weight);
        self.terminal_weights_sum += weight;
        
        return Ok(self);
    }
    
    /// Builds final operator set with validation.
    ///
    /// # Returns
    /// * `Result<Operators, BuilderError>` - Valid [`operator set`][Operators] or
    /// [`error`][`BuilderError`]
    pub fn build(self) -> Result<Operators, BuilderError> {
        if self.functions.is_empty() { return Err(BuilderError::OperatorsIsEmpty); }
        if (self.function_weights_sum - 1.0).abs() > 1e-10 { return Err(BuilderError::WrongWeightSum); }
        if self.terminals.is_empty() { return Err(BuilderError::OperatorsIsEmpty); }
        if (self.terminal_weights_sum - 1.0).abs() > 1e-10 { return Err(BuilderError::WrongWeightSum); }
        
        // Build function sampler
        let capacity = self.functions.len();
        let (mut ops, mut arity, mut weights) = 
            (Vec::with_capacity(capacity), Vec::with_capacity(capacity), Vec::with_capacity(capacity));

        for (name, func) in &self.functions {
            ops.push(name.clone());
            arity.push(func.arity());
            weights.push(func.weight());
        }
        
        let function_sampler = OperatorSampler::new(ops, arity, weights);
        
        // Build terminal sampler (simplified for terminals since they all have arity 0)
        let terminal_names: Vec<String> = self.terminals.iter()
            .map(|node| node.name())
            .collect();
        let terminal_arities = vec![0; self.terminals.len()];
        
        let terminal_sampler = OperatorSampler::new(terminal_names, terminal_arities, self.terminal_weights);

        return Ok(Operators::new(
            self.functions, 
            self.terminals, 
            function_sampler, 
            terminal_sampler
        ));
    }
}

impl Default for OperatorsBuilder {
    fn default() -> Self { 
        return Self { 
            functions: HashMap::new(), 
            terminals: Vec::new(),
            terminal_weights: Vec::new(),
            function_weights_sum: 0.0,
            terminal_weights_sum: 0.0,
        }; 
    }
}