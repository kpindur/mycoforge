//! Core dataset structures for handling training and test data.
use rand::Rng;
use rand::seq::index::sample;

use std::collections::HashSet;

use crate::common::traits::Data;
use crate::dataset::error::DatasetError;

use super::loaders::csv_loader::load_csv;

//enum TrainSelection {
//    Random,
//    Systematic,
//    Stratified
//}

/// Dataset structure holding feature names and split data vectors.
///
/// # Fields
/// * `feature_names: Vec<String>` - names of features in dataset
/// * `no_dims: usize` - number of input dimensions (excluding target)
/// * `test_data: Vec<Vec<f64>>` - test set feature vectors
/// * `train_data: Vec<Vec<f64>>` - training set feature vectors
pub struct Dataset {
    feature_names: Vec<String>,
    no_dims: usize,
    test_data: Vec<Vec<f64>>,
    train_data: Vec<Vec<f64>>,
}

impl Dataset {
    /// Creates new dataset with provided fields.
    ///
    /// # Arguments
    /// * `feature_names: Vec<String>` - names of features
    /// * `no_dims: usize` - number of input dimensions
    /// * `test_data` - test set feature vectors
    /// * `train_data` - training set feature vectors
    pub fn new(feature_names: Vec<String>, no_dims: usize, test_data: Vec<Vec<f64>>, train_data: Vec<Vec<f64>>) -> Self {
        return Self { feature_names, no_dims, test_data, train_data }
    }

    /// Loads dataset from CSV file and splits into train/test sets.
    ///
    /// # Arguments
    /// * `rng: &mut R` - [`random number generator`][`rand::Rng`]
    /// * `path: &str` - path to csv file
    /// * `test_ratio: f64` - ratio of data to use for testing
    ///
    /// # Returns
    /// * `Result<Self, DatasetError>` - new dataset or error if loading fails
    pub fn from_csv<R: Rng>(rng: &mut R, path: &str, test_ratio: f64) -> Result<Self, DatasetError> {
        let (header, columns) = load_csv(path.as_ref())?;

        return Ok(Self::from_vector(rng, header, columns, test_ratio));
    }

    /// Creates dataset from vector data and splits into train/test sets.
    ///
    /// # Arguments
    /// * `rng: &mut R` - [`random number generator`][`rand::Rng`]
    /// * `feature_names` - names of features
    /// * `vectors: Vec<Vec<f64>>` - feature vectors
    /// * `test_ratio: f64` - ratio of data to use for testing
    pub fn from_vector<R: Rng>(rng: &mut R, feature_names: Vec<String>, vectors: Vec<Vec<f64>>, test_ratio: f64) -> Self {
        let no_dims = vectors.len() - 1;
        
        let (test_data, train_data) = Self::separate(rng, &vectors, test_ratio);

        return Self { feature_names, no_dims, test_data, train_data };
    }

    pub fn feature_names(&self) -> &Vec<String> { return &self.feature_names; }
    pub fn test_data(&self)     -> &Vec<Vec<f64>> { return &self.test_data; }
    pub fn train_data(&self)    -> &Vec<Vec<f64>> { return &self.train_data; }

    /// Separates data into train and test sets.
    ///
    /// # Arguments
    /// * `rng: &mut R` - [`random number generator`][`rand::Rng`]
    /// * `data: &[Vec<f64>]` - data to split
    /// * `test_ration: f64` - ratio of data to use for testing
    ///
    /// # Returns
    /// * `(Vec<Vec<f64>>, Vec<Vec<f64>>)` - (test_data, train_data) tuple
    fn separate<R: Rng>(rng: &mut R, data: &[Vec<f64>], test_ratio: f64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n_samples = data[0].len();
        let test_size = (n_samples as f64 * test_ratio).ceil() as usize;
        
        let test_set = sample(rng, n_samples, test_size).into_iter().collect::<HashSet<usize>>();

        let mut test_data = vec![Vec::with_capacity(test_size); data.len()];
        let mut train_data = vec![Vec::with_capacity(n_samples - test_size); data.len()];

        for i in 0..data[0].len() {
            for (j, dat) in data.iter().enumerate() {
                if test_set.contains(&i) { test_data[j].push(dat[i]); }
                else { train_data[j].push(dat[i])}
            }
        }

        return (test_data, train_data);
    }
}

impl Data for Dataset {
    fn data_test(&self)   -> (usize, &Vec<Vec<f64>>) { return (self.no_dims, self.test_data()); }
    fn data_train(&self)  -> (usize, &Vec<Vec<f64>>) { return (self.no_dims, self.train_data()); }
}
