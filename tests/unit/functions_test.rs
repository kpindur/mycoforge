
use mycoforge::operators::functions::symbolic::*;
use mycoforge::operators::set::{OperatorsBuilder, BuilderError};

#[test]
fn test_empty_input() {
    let empty: Vec<f64> = Vec::new();
    let nonempty = [1.0f64];

    assert!(div(&[&empty, &nonempty]).is_empty());
    assert!(div(&[&nonempty, &empty]).is_empty());
    assert!(sin(&[&empty]).is_empty());
}

#[test]
fn test_div_by_zero() {
    let a = [1.0f64];
    let b = [0.0f64];
    let result = div(&[&a, &b]);

    assert_eq!(1.0, result[0]);
}

#[test]
fn test_overflow_mul() {
    let a = [1e308f64];
    let result = mul(&[&a, &a]);

    assert!(result[0].is_finite());
    assert_eq!(1e10, result[0]);
}

#[test]
fn test_infinite_trig() {
    let a = [f64::INFINITY];
    let sin_result = sin(&[&a]);
    let cos_result = cos(&[&a]);

    assert_eq!(0.0, sin_result[0]);
    assert_eq!(0.0, cos_result[0]);
}

#[test]
fn test_ln_zero() {
    let a = [0.0f64];
    let result = ln(&[&a]);

    assert!(result[0].is_finite());
}

#[test]
fn test_full_set_works() -> Result<(), BuilderError>  {
    let operators = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 7.0)?
        .add_operator("-", sub, 2, 1.0 / 7.0)?
        .add_operator("*", mul, 2, 1.0 / 7.0)?
        .add_operator("/", div, 2, 1.0 / 7.0)?
        .add_operator("sin", sin, 1, 1.0 / 7.0)?
        .add_operator("cos", cos, 1, 1.0 / 7.0)?
        .add_operator("ln", ln, 1, 1.0 / 7.0)?
        .build().expect("Failed to build operators!");
    
    assert_eq!(7, operators.operators().len(),
        "Operators length is incorrect! Expected: {} found {}", 7, operators.operators().len());

    return Ok(());
}
