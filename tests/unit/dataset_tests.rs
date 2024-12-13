use rand::{rngs::StdRng, thread_rng, SeedableRng};

use mycoforge::dataset::core::Dataset;

fn sample_data() -> (Vec<String>, Vec<Vec<f64>>) {
    let headers = vec!["x".to_string(), "y".to_string()];
    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 4.0, 6.0, 8.0, 10.0], 
    ];

    return (headers, data);
}

fn sample_load_data(ratio: f64) -> Dataset {
    return Dataset::from_csv(
        &mut thread_rng(), "tests/fixtures/test_f1.csv",
        ratio)
        .expect("Failed to load data!");
}

#[test]
fn test_manual_creation() {
    let (headers, data) = sample_data();
    let dataset = Dataset::new(headers.clone(), 1,
        vec![vec![1.0], vec![2.0]],
        data
    );

    assert_eq!(dataset.feature_names(), &headers);
    assert_eq!(dataset.feature_names().len() - 1, 1);
}

#[test]
fn test_separation() {
    let ratios = [0.2, 0.3, 0.4, 0.5];
    let whole_dataset = sample_load_data(0.0);
    for ratio in ratios {
        let dataset = sample_load_data(ratio);

        assert_eq!(dataset.test_data().len(), dataset.train_data().len(),
            "Number of columns does not match! Expected {}, found {}", dataset.test_data().len(), dataset.train_data().len()
        );

        let total = dataset.test_data()[0].len() + dataset.train_data()[0].len();
        assert_eq!(total, whole_dataset.train_data()[0].len(),
            "Total number of datapoints does not match! Expected {}, found {}", total, whole_dataset.train_data()[0].len()
        );
    }
}

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
