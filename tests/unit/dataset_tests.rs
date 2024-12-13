use rand::{SeedableRng, rngs::StdRng};

use mycoforge::dataset::core::Dataset;

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
