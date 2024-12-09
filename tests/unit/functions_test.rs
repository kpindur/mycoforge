
use mycoforge::operators::functions::symbolic::*;
use mycoforge::operators::set::{OperatorsBuilder, BuilderError};

#[test]
fn test_full_set_works() -> Result<(), BuilderError>  {
    let _ = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0 / 7.0)?
        .add_operator("-", sub, 2, 1.0 / 7.0)?
        .add_operator("*", mul, 2, 1.0 / 7.0)?
        .add_operator("/", div, 2, 1.0 / 7.0)?
        .add_operator("sin", sin, 1, 1.0 / 7.0)?
        .add_operator("cos", cos, 1, 1.0 / 7.0)?
        .add_operator("ln", ln, 1, 1.0 / 7.0)?
        .build();
    
    return Ok(());
}
