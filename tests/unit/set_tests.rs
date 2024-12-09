use mycoforge::operators::set::BuilderError;
use mycoforge::operators::set::OperatorsBuilder;

use mycoforge::operators::functions::symbolic::add;


fn x(vec: &[&[f64]]) -> Vec<f64> {
    return vec[0].to_vec();
}

#[test]
fn test_builder_works() -> Result<(), BuilderError> {
    let ops = OperatorsBuilder::default()
        .add_operator("+", add, 2, 0.5)?
        .add_operator("x", x, 0, 0.5)?
        .build()?;
    assert_eq!(2, ops.operators().len());

    let duplicate = OperatorsBuilder::default()
        .add_operator("+", add, 2, 0.5)?
        .add_operator("+", add, 2, 0.5);
    assert!(duplicate.is_err());

    let wrong_weights = OperatorsBuilder::default()
        .add_operator("+", add, 2, 1.0)?
        .add_operator("x", x, 0, 0.25)?
        .build();
    assert!(wrong_weights.is_err());

    return Ok(());
}
