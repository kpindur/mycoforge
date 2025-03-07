//! Functions and operators for symbolic regression.
//!
//! This module provides:
//! - Generic trait for floating point operations
//! - Basic arithmetic operators (+, -, *, /)
//! - Trigonometric functions (sin, cos)
//! - Natural logarithm
pub mod symbolic {
    use std::cmp::PartialOrd;
    use std::ops::{Add, Sub, Mul, Div};
    /// Trait for abstracting over floating point types in symbolic expressions.
    ///
    /// Provides common mathematical operations and constants.
    pub trait Float: Copy + PartialOrd {
        /// Return zero value
        fn zero() -> Self;
        /// Return one value
        fn one() -> Self;
        
        /// Returns minimum allowed value
        fn min_value() -> Self;
        /// Returns maximum allowed value
        fn max_value() -> Self;
        
        /// Returns epsilon for minimum division value
        fn epsilon() -> Self;
        
        /// Computes sine
        fn sin(self) -> Self;
        /// Computes cosine
        fn cos(self) -> Self;
        /// Computes natural logarithm
        fn ln(self) -> Self;

        /// Calculate absolute value
        fn abs(self) -> Self;
        /// Check if the value is finite
        fn is_finite(self) -> bool;
        /// Returns sign of value
        fn signum(self) -> Self;
    }

    impl Float for f32 {
        fn zero() -> Self { return 0.0; }
        fn one() -> Self { return 1.0; }

        fn min_value() -> Self { return f32::MIN; }
        fn max_value() -> Self { return 1e10; }

        fn epsilon() -> Self { return 1e-6; }
        fn sin(self) -> Self { return self.sin(); }
        fn cos(self) -> Self { return self.cos(); }
        fn ln(self) -> Self { return self.ln(); }

        fn abs(self) -> Self { return self.abs(); }
        fn is_finite(self) -> bool { return self.is_finite(); }
        fn signum(self) -> Self { return self.signum(); }
    }

    impl Float for f64 {
        fn zero() -> Self { return 0.0; }
        fn one() -> Self { return 1.0; }

        fn min_value() -> Self { return f64::MIN; }
        fn max_value() -> Self { return 1e10; }

        fn epsilon() -> Self { return 1e-6; }
        fn sin(self) -> Self { return self.sin(); }
        fn cos(self) -> Self { return self.cos(); }
        fn ln(self) -> Self { return self.ln(); }

        fn abs(self) -> Self { return self.abs(); }
        fn is_finite(self) -> bool { return self.is_finite(); }
        fn signum(self) -> Self { return self.signum(); }
    }

    /// Helper type aliases for operator functions
    type UnaryOp<T> = fn(T) -> T;
    type BinaryOp<T> = fn(T, T) -> T;
    
    /// Applies unary operation to vector of values
    fn apply_unary<T: Float>(op: UnaryOp<T>, args: &[&[T]]) -> Vec<T> {
        if args.len() != 1 || args[0].is_empty() {
            return Vec::new();
        }
        return args[0].iter().map(|&a| op(a)).collect();
    }

    /// Applies binary operation to paired values from two vectors
    fn apply_binary<T: Float>(op: BinaryOp<T>, args: &[&[T]]) -> Vec<T> {
        if args.len() != 2 || args[0].is_empty() || args[1].is_empty() {
            return Vec::new();
        }
        return args[0].iter().zip(args[1].iter())
            .map(|(&a, &b)| op(a, b))
            .collect();
    }

    /// Addition operator
    pub fn add<T: Add<Output = T> + Float>(args: &[&[T]]) -> Vec<T> { 
        return apply_binary(|a, b| a + b, args); 
    }
    
    /// Subtraction operator
    pub fn sub<T: Sub<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| a - b, args); 
    }

    /// Multiplication operator with overflow protection
    pub fn mul<T: Mul<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| {
            let result = a * b;
            if result.is_finite() { result } else { result.signum() * T::max_value() }
        }, args);
    }

    /// Protected division operator (returns 1.0 for division by values smaller than epsilon)
    pub fn div<T: Div<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| if b.abs() < T::epsilon() { return T::one(); } else { return a / b }, args);
    }

    /// Protected sine operator (returns 0.0 for non-finite inputs)
    pub fn sin<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a.is_finite() { a.sin() } else { T::zero() }, args);
    }

    /// Protected cosine operator (returns 0.0 for non-finite inputs)
    pub fn cos<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a.is_finite() { a.cos() } else { T::zero() }, args);
    }
    
    /// Protected natural logarithm (return minimum value for inputs less than or equal to epsilon)
    pub fn ln<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a > T::epsilon() { a.ln() } else { T::min_value() }, args);
    }
}

use crate::operators::builder::{BuilderError, OperatorsBuilder};
use symbolic::{add, sub, mul, div, sin, cos};

/// Creates standard Koza function set for symbolic regression.
///
/// # Arguments
/// * `operators_size: usize` - total number of operators (must be greater than 6 for koza set).
/// Total number of operators includes terminal operators (number of variables).
///
/// # Returns
/// * `Result<OperatorsBuilder, BuilderError>` - Builder including Koza operators
pub fn koza(operators_size: usize) -> Result<OperatorsBuilder, BuilderError> {
    assert!(operators_size > 6, "Operators size too small! Expected more than {}, found {}", 6, operators_size);
    let operators_size = operators_size as f64;
    let koza = OperatorsBuilder::default()
        .add_function("+", add, 2, 1.0 / operators_size).expect("Failed to add an operator!")
        .add_function("-", sub, 2, 1.0 / operators_size).expect("Failed to add an operator!")
        .add_function("*", mul, 2, 1.0 / operators_size).expect("Failed to add an operator!")
        .add_function("/", div, 2, 1.0 / operators_size).expect("Failed to add an operator!")
        .add_function("sin", sin, 1, 1.0 / operators_size).expect("Failed to add an operator!")
        .add_function("cos", cos, 1, 1.0 / operators_size).expect("Failed to add an operator!");

    return Ok(koza);
}
