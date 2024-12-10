pub mod symbolic {
    use std::cmp::PartialOrd;
    use std::ops::{Add, Sub, Mul, Div};

    pub trait Float: Copy + PartialOrd {
        fn zero() -> Self;
        fn one() -> Self;

        fn min_value() -> Self;
        fn max_value() -> Self;

        fn epsilon() -> Self;

        fn sin(self) -> Self;
        fn cos(self) -> Self;
        fn ln(self) -> Self;

        fn is_finite(self) -> bool;
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

        fn is_finite(self) -> bool { return self.is_finite(); }
        fn signum(self) -> Self { return self.signum(); }
    }

    type UnaryOp<T> = fn(T) -> T;
    type BinaryOp<T> = fn(T, T) -> T;

    fn apply_unary<T: Float>(op: UnaryOp<T>, args: &[&[T]]) -> Vec<T> {
        if args.len() != 1 || args[0].is_empty() {
            return Vec::new();
        }
        return args[0].iter().map(|&a| op(a)).collect();
    }


    fn apply_binary<T: Float>(op: BinaryOp<T>, args: &[&[T]]) -> Vec<T> {
        if args.len() != 2 || args[0].is_empty() || args[1].is_empty() {
            return Vec::new();
        }
        return args[0].iter().zip(args[1].iter())
            .map(|(&a, &b)| op(a, b))
            .collect();
    }

    pub fn add<T: Add<Output = T> + Float>(args: &[&[T]]) -> Vec<T> { 
        return apply_binary(|a, b| a + b, args); 
    }

    pub fn sub<T: Sub<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| a - b, args); 
    }

    pub fn mul<T: Mul<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| {
            let result = a * b;
            if result.is_finite() { result } else { result.signum() * T::max_value() }
        }, args);
    }

    pub fn div<T: Div<Output = T> + Float>(args: &[&[T]]) -> Vec<T> {
        return apply_binary(|a, b| if b < T::epsilon() { return T::one(); } else { return a / b }, args);
    }

    pub fn sin<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a.is_finite() { a.sin() } else { T::zero() }, args);
    }

    pub fn cos<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a.is_finite() { a.cos() } else { T::zero() }, args);
    }

    pub fn ln<T: Float>(args: &[&[T]]) -> Vec<T> {
        return apply_unary(|a| if a > T::epsilon() { a.ln() } else { T::min_value() }, args);
    }
}

use crate::operators::set::{BuilderError, OperatorsBuilder, Operators};
use symbolic::{add, sub, mul, div, sin, cos};

pub fn koza() -> Result<Operators, BuilderError> {
    let koza = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 6.0)?
        .add_operator("-", sub, 2, 1.0 / 6.0)?
        .add_operator("*", mul, 2, 1.0 / 6.0)?
        .add_operator("/", div, 2, 1.0 / 6.0)?
        .add_operator("sin", sin, 1, 1.0 / 6.0)?
        .add_operator("cos", cos, 1, 1.0 / 6.0)?
        .build();

    return koza;
}
