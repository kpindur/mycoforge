use std::fmt::{Display, Formatter, Result};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use mycoforge::common::traits::{Genotype, Individual};
use mycoforge::tree::core::individual::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
struct MockGenotype;
impl Genotype for MockGenotype {}

impl Display for MockGenotype {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "MockGenotype")
    }
}

#[test]
fn test_individual_creation() {
    let ind = TreeIndividual::new(MockGenotype, 1.5);
    assert_eq!(*ind.genotype(), MockGenotype);
    assert_eq!(ind.phenotype(), 1.5);
}

#[test]
fn test_from_vecs() {
    let genotypes = vec![MockGenotype, MockGenotype];
    let fitnesses = vec![1.0, 2.0];
    let individuals = TreeIndividual::from_vecs(&genotypes, &fitnesses);

    assert_eq!(2, individuals.len());
    assert_eq!(1.0, individuals[0].phenotype());
    assert_eq!(2.0, individuals[1].phenotype());
}

#[test]
fn test_from_genotype_vec() {
    let genotypes = vec![MockGenotype, MockGenotype];
    let individuals = TreeIndividual::from_genotype_vec(&genotypes);

    assert_eq!(individuals.len(), 2);
    assert_eq!(individuals[0].phenotype(), f64::NEG_INFINITY);
}

#[test]
fn test_to_genotype_vec() {
    let individuals = vec![
        TreeIndividual::new(MockGenotype, 1.0),
        TreeIndividual::new(MockGenotype, 2.0)
    ];
    let genotypes = TreeIndividual::to_genotype_vec(&individuals);

    assert_eq!(2, genotypes.len());
    assert_eq!(MockGenotype, genotypes[0]);
}

#[cfg(feature = "serde")]
#[test]
fn test_tree_individual_serialization() {
    use serde_json;
    let individual = TreeIndividual::new(MockGenotype, 42.0);

    let serialized = serde_json::to_string(&individual)
        .expect("Failed to serialize individual!");
    let deserialized: TreeIndividual<MockGenotype> = serde_json::from_str(&serialized)
        .expect("Failed to deserialzie individual!");

    assert_eq!(*deserialized.genotype(), MockGenotype,
        "Deserialized genotype is different! Expected {}, found {}",
        MockGenotype, *deserialized.genotype()
    );
    assert_eq!(deserialized.phenotype(), 42.0,
        "Deserialized phenotype is different! Expected {}, found {}",
        42.0, deserialized.phenotype()
    );

    let individuals = vec![
        TreeIndividual::new(MockGenotype, 1.0),
        TreeIndividual::new(MockGenotype, 2.0)
    ];

    let serialized = serde_json::to_string(&individuals)
        .expect("Failed to serialize individuals!");
    let deserialized: Vec<TreeIndividual<MockGenotype>> = serde_json::from_str(&serialized)
        .expect("Failed to deserialize individuals!");

    for value in &deserialized {
        assert_eq!(*value.genotype(), MockGenotype,
            "Deserialized genotype is different! Expected {}, found {}",
            MockGenotype, *value.genotype()
        );
    }
    assert_eq!(deserialized[0].phenotype(), 1.0, 
        "Deserialized phenotype is different! Expected {}, found {}", 
        1.0, deserialized[0].phenotype()
    );
    assert_eq!(deserialized[1].phenotype(), 2.0, 
        "Deserialized phenotype is different! Expected {}, found {}", 
        2.0, deserialized[1].phenotype()
    );
}
