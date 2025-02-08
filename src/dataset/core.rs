//! Core dataset structures for handling training and test data.
use std::collections::HashMap;

use crate::common::traits::Data;
use crate::dataset::error::DatasetError;

use super::loaders::csv_loader::{load_csv, load_csv_with_metadata};
use super::loaders::parquet_loader::load_parquet;

pub type OutputData = (Vec<String>, String, Vec<Vec<f64>>, Vec<f64>);
pub type Metadata = HashMap<String, String>;

/// Dataset structure holding feature names and split data vectors.
///
/// # Fields
/// * `feature_names: Vec<String>` - names of features in Dataset
/// * `target_name: String` - name of the target in Dataset
/// * `features: Vec<Vec<f64>>` - n-dimensional array of features
/// * `targets: Vec<f64>` - 1-dimensional array of targets
pub struct Dataset {
    metadata: Option<Metadata>,
    feature_names: Vec<String>,
    target_name: String,
    features: Vec<Vec<f64>>,
    targets: Vec<f64>
}

impl Dataset {
    /// Creates new dataset with provided fields.
    ///
    /// # Arguments
    /// * `feature_names: Vec<String>` - names of features
    /// * `target_name: String` - name of the target
    /// * `features: Vec<Vec<f64>>` - n-dimensional array of features
    /// * `targets: Vec<f64>` - 1-dimensional array of targets
    pub fn new(
        metadata: Option<Metadata>,
        feature_names: Vec<String>, target_name: String, 
        features: Vec<Vec<f64>>, targets: Vec<f64>
    ) -> Self {
        return Self { metadata, feature_names, target_name, features, targets };
    }

    /// Loads dataset from CSV file.
    ///
    /// # Arguments
    /// * `path: &str` - path to csv file
    /// * `n_features: usize` - number of features in dataset
    ///
    /// # Returns
    /// * `Result<Self, DatasetError>` - new dataset or error if loading fails
    pub fn from_csv(path: &str, n_features: usize) -> Result<Self, DatasetError> {
        let (feature_names, target_name, 
            features, targets) = load_csv(path, n_features)?;

        return Ok(Self::from_vector(None, feature_names, target_name, features, targets));
    }

    /// Loads dataset from CSV file, which includes metadata
    ///
    /// # Arguments
    /// * `path: &str` - path to csv file
    /// * `n_features: usize` - number of features in dataset
    ///
    /// # Returns
    /// * `Result<(), DatasetError>` - new dataset or error if loading fails
    pub fn from_csv_with_metadata(path: &str, n_features: usize) -> Result<Self, DatasetError> {
        let (metadata, 
            (feature_names, target_name, 
             features, targets)) = load_csv_with_metadata(path, n_features)?;

        return Ok(Self::from_vector(Some(metadata), feature_names, target_name, features, targets));
    }

    /// Loads dataset from Parquet file.
    ///
    /// # Arguments
    /// * `path: &str` - path to parquet file
    ///
    /// # Returns
    /// * `Result<Self, DatasetError>` - new dataset or error if loading fails
    pub fn from_parquet(path: &str) -> Result<Self, DatasetError> {
        let (feature_names, target_name, features, targets) = load_parquet(path)?;

        return Ok(Self::from_vector(None, feature_names, target_name, features, targets));
    }

    fn from_vector(
        metadata: Option<Metadata>,
        feature_names: Vec<String>, target_name: String,
        features: Vec<Vec<f64>>, targets: Vec<f64>
    ) -> Self {
        return Self { metadata, feature_names, target_name, features, targets };
    }

    pub fn metadata(&self) -> &Option<Metadata> { return &self.metadata; }
    pub fn feature_names(&self) -> &Vec<String> { return &self.feature_names; }
    pub fn target_name(&self) -> &String { return &self.target_name; }
    pub fn features(&self) -> &Vec<Vec<f64>> { return &self.features; }
    pub fn targets(&self) -> &Vec<f64> { return &self.targets; }
}

impl Data for Dataset {
    fn names(&self) -> (&Vec<String>, &String) { return (&self.feature_names, &self.target_name); }
    fn data(&self) -> (&Vec<Vec<f64>>, &Vec<f64>) { return (&self.features, &self.targets); }
}
