//! Core dataset structures for handling training and test data.
use crate::common::traits::Data;
use crate::dataset::error::DatasetError;

use super::loaders::csv_loader::load_csv;


/// Dataset structure holding feature names and split data vectors.
///
/// # Fields
/// * `feature_names: Vec<String>` - names of features in dataset
/// * `no_dims: usize` - number of input dimensions (excluding target)
/// * `test_data: Vec<Vec<f64>>` - test set feature vectors
/// * `train_data: Vec<Vec<f64>>` - training set feature vectors
pub struct Dataset {
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
    /// * `no_dims: usize` - number of input dimensions
    /// * `data` - stored data, includes both features and truth vectors.
    pub fn new(feature_names: Vec<String>, target_name: String, features: Vec<Vec<f64>>, targets: Vec<f64>) -> Self {
        return Self { feature_names, target_name, features, targets };
    }

    /// Loads dataset from CSV file.
    ///
    /// # Arguments
    /// * `path: &str` - path to csv file
    ///
    /// # Returns
    /// * `Result<Self, DatasetError>` - new dataset or error if loading fails
    pub fn from_csv(path: &str, n_features: usize) -> Result<Self, DatasetError> {
        let (feature_names, target_name, features, targets) = load_csv(path, n_features)?;

        return Ok(Self::from_vector(feature_names, target_name, features, targets));
    }

    /// Creates dataset from vector data.
    ///
    /// # Arguments
    /// * `feature_names` - names of features
    /// * `vectors: Vec<Vec<f64>>` - feature vectors
    pub fn from_vector(
        feature_names: Vec<String>, target_name: String,
        features: Vec<Vec<f64>>, targets: Vec<f64>
    ) -> Self {
        return Self { feature_names, target_name, features, targets };
    }

}

impl Data for Dataset {
    fn names(&self) -> (&Vec<String>, &String) { return (&self.feature_names, &self.target_name); }
    fn data(&self) -> (&Vec<Vec<f64>>, &Vec<f64>) { return (&self.features, &self.targets); }
}
