#![allow(clippy::needless_return)]

pub mod common;

pub mod operators;

pub mod dataset;

#[cfg(not(feature = "postgres"))]
pub use crate::dataset::logger::SimpleLogger as Logger;
#[cfg(feature = "postgres")]
pub use crate::dataset::logger::PostgresLogger as Logger;

//pub mod linear;

pub mod tree;

//pub mod graph;

//pub mod grammatical;

//pub mod utils;

//pub mod population;

pub mod optimizers;
