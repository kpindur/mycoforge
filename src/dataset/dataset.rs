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

pub struct Dataset {
    feature_names: Vec<String>,
    no_dims: usize,
    test_data: Vec<Vec<f64>>,
    train_data: Vec<Vec<f64>>,
}

impl Dataset {
    pub fn new(feature_names: Vec<String>, no_dims: usize, test_data: Vec<Vec<f64>>, train_data: Vec<Vec<f64>>) -> Self {
        return Self { feature_names, no_dims, test_data, train_data }
    }

    pub fn from_csv<R: Rng>(rng: &mut R, path: &str, test_ratio: f64) -> Result<Self, DatasetError> {
        let (header, columns) = load_csv(path.as_ref())?;

        return Ok(Self::from_vector(rng, header, columns, test_ratio));
    }

    pub fn from_vector<R: Rng>(rng: &mut R, feature_names: Vec<String>, vectors: Vec<Vec<f64>>, test_ratio: f64) -> Self {
        let no_dims = vectors.len() - 1;
        
        let (test_data, train_data) = Self::separate(rng, &vectors, test_ratio);

        return Self { feature_names, no_dims, test_data, train_data };
    }

    pub fn feature_names(&self) -> &Vec<String> { return &self.feature_names; }
    pub fn test_data(&self)     -> &Vec<Vec<f64>> { return &self.test_data; }
    pub fn train_data(&self)    -> &Vec<Vec<f64>> { return &self.train_data; }

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

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_creation() {
        let mut rng = StdRng::seed_from_u64(102);

        let feature_names = ["x", "y", "z"].iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let data_x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let data_y = vec![0.0, -1.0, -2.0, -3.0, -4.0, -5.0];
        let data_z = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];

        let train_ratio = 0.1;
        let dataset = Dataset::from_vector(&mut rng, feature_names, vec![data_x, data_y, data_z], train_ratio);
        println!("Dataset.test_data: {:?}", dataset.test_data());
        println!("Dataset.train_data: {:?}", dataset.train_data());
        // Any sensible test case?
        //assert_eq!(&data_out, dataset.data());
    }
}
