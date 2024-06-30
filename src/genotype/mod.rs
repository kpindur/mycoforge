/// `init.rs` contains all initialization methods
/// For the time being: InitUniform, InitFromDistribution
pub mod init;
/// `mutation.rs` contains all mutation methods
/// For the time being: UniformBinaryMutation
pub mod mutation;
/// `crossover.rs` contains all crossover methods
pub mod crossover;

/// `enums.rs` contains enum types for initialization, mutation and crossover.
pub mod enums;
/// `genotype.rs` contains both linear and non-linear genotype definitions
pub mod genotype;

