use mycoforge::operators::functions::symbolic::*;
use mycoforge::operators::functions::koza;
use mycoforge::operators::set::{OperatorsBuilder, BuilderError};

#[test]
fn test_f32_functions() {
    let epsilon_f32 = 1e-6f32;
    let relative_error = (epsilon_f32 - f32::epsilon()) / epsilon_f32;
    assert!(relative_error < 1e-10,
        "f32::epsilon: expected {}, found {} with relative error {}",
        epsilon_f32, f32::epsilon(), relative_error
    );
    assert!(1.0 - f32::one() < epsilon_f32,
        "f32::one(): expected {}, found {} ", 1.0, f32::one()
    );
    assert!(0.0 - f32::zero() < epsilon_f32,
        "f32::zero(): expected {}, found {}", 0.0, f32::zero()
    );
    let relative_error = (f32::MIN - f32::min_value()) / f32::MIN;
    assert!(relative_error < epsilon_f32,
        "f32::min_value: expected {}, found {}, with relative error {}",
        f32::MIN, f32::min_value(), relative_error
    );
    let max_val = 1e10;
    let relative_error = (max_val - f32::max_value()) / f32::MAX;
    assert!(relative_error < epsilon_f32,
        "f32::max_value: expected {}, found {}, with relative error {}",
        f32::MAX, f32::max_value(), relative_error
    );
    let test_value = 0.5f32;
    assert!(test_value.sin() - Float::sin(0.5) < epsilon_f32,
        "f32::sin: expected {}, found {}",
        test_value.sin(), Float::sin(0.5)
    );
    assert!(test_value.cos() - Float::cos(0.5) < epsilon_f32,
        "f32::cos: expected {}, found {}",
        test_value.cos(), Float::cos(0.5)
    );
    assert!(test_value.ln() - Float::ln(0.5) < epsilon_f32,
        "f32::ln: expected {}, found {}",
        test_value.ln(), Float::ln(0.5)
    );
}

#[test]
fn test_f64_functions() {
    let epsilon_f64 = 1e-6f64;
    let relative_error = (epsilon_f64 - f64::epsilon()) / epsilon_f64;
    assert!(relative_error < 1e-10,
        "f64::epsilon: expected {}, found {} with relative error {}",
        epsilon_f64, f64::epsilon(), relative_error
    );
    assert!(1.0 - f64::one() < epsilon_f64,
        "f64::one(): expected {}, found {} ", 1.0, f64::one()
    );
    assert!(0.0 - f64::zero() < epsilon_f64,
        "f64::zero(): expected {}, found {}", 0.0, f64::zero()
    );
    let relative_error = (f64::MIN - f64::min_value()) / f64::MIN;
    assert!(relative_error < epsilon_f64,
        "f64::min_value: expected {}, found {}, with relative error {}",
        f64::MIN, f64::min_value(), relative_error
    );
    let max_val = 1e10;
    let relative_error = (max_val - f64::max_value()) / f64::MAX;
    assert!(relative_error < epsilon_f64,
        "f64::max_value: expected {}, found {}, with relative error {}",
        f64::MAX, f64::max_value(), relative_error
    );
    let test_value = 0.5f64;
    assert!(test_value.sin() - Float::sin(0.5) < epsilon_f64,
        "f64::sin: expected {}, found {}",
        test_value.sin(), Float::sin(0.5)
    );
    assert!(test_value.cos() - Float::cos(0.5) < epsilon_f64,
        "f64::cos: expected {}, found {}",
        test_value.cos(), Float::cos(0.5)
    );
    assert!(test_value.ln() - Float::ln(0.5) < epsilon_f64,
        "f64::ln: expected {}, found {}",
        test_value.ln(), Float::ln(0.5)
    );
}

fn x(vec: &[&[f64]]) -> Vec<f64> { return vec[0].to_vec(); }

#[test]
fn test_koza_builder() {
    let builder = koza(7).expect("Failed to construct builder for koza set!");
    let operators = builder
        .add_operator("x", x, 0, 1.0 / 7.0).expect("Failed to add an operator!")
        .build().expect("Failed to build an operator set!");

    assert_eq!(operators.operators().len(), 7,
        "Wrong operators size! Expected {}, found {}", 7, operators.operators().len()
    );
}

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
