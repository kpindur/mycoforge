use std::fs::File;
use std::collections::HashMap;
use arrow::array::Float64Array;
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;

use mycoforge::{common::traits::Data, dataset::core::Dataset};

fn sample_data() -> (Vec<String>, Vec<Vec<f64>>) {
    let headers = vec!["x".to_string(), "y".to_string()];
    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 4.0, 6.0, 8.0, 10.0], 
    ];

    return (headers, data);
}

fn setup_test_parquet_data(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if std::path::Path::new(path).exists() {
        return Ok(());
    }

    let feature1 = Float64Array::from(vec![1.0, 2.0, 3.0]);
    let feature2 = Float64Array::from(vec![4.0, 5.0, 6.0]);
    let target = Float64Array::from(vec![7.0, 8.0, 9.0]);

        let schema = Schema::new(vec![
        Field::new("f1", DataType::Float64, false),
        Field::new("f2", DataType::Float64, false),
        Field::new("target", DataType::Float64, false),
    ]);
    
    let batch = RecordBatch::try_new(
        std::sync::Arc::new(schema),
        vec![
           std::sync::Arc::new(feature1),
           std::sync::Arc::new(feature2),
           std::sync::Arc::new(target),
        ],
    )?;
    
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
    writer.write(&batch)?;
    writer.close()?;

    return Ok(());
}

#[test]
fn test_manual_creation() {
    let (headers, data) = sample_data();

    let features = vec![data[0].clone()];
    let targets = data[1].clone();
    let dataset = Dataset::new(headers[0..headers.len()-2].to_vec(), headers[headers.len()-1].clone(),
        features, targets
    );

    let (feature_names, target_names) = dataset.names();

    assert_eq!(feature_names, &headers[0..headers.len()-2].to_vec(),
        "Stored feature names are different! Expected: {:?}, found {:?}",
        feature_names, &headers[0..headers.len()-2]
    );
    assert_eq!(target_names, &headers[headers.len()-1],
        "Stored target name is different! Expected: {}, Found {}",
        target_names, headers.last().unwrap()
    );
}

#[test]
fn test_creation() {
    let feature_names = ["x", "y"].iter().map(|v| v.to_string()).collect::<Vec<String>>();
    let target_name = "z".to_string();

    let data_x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let data_y = vec![0.0, -1.0, -2.0, -3.0, -4.0, -5.0];
    let data_z = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];

    let features = vec![data_x, data_y];
    let targets = data_z;

    let dataset = Dataset::new(
        feature_names.clone(), target_name.clone(), 
        features.clone(), targets.clone()
    );

    assert_eq!(&feature_names, dataset.names().0,
        "Stored feature names are different! Expected {:?}, found {:?}",
        feature_names, dataset.names().0
    );

    assert_eq!(&target_name, dataset.names().1,
        "Stored feature names are different! Expected {:?}, found {:?}",
        target_name, dataset.names().1
    );

    assert_eq!(&features, dataset.data().0,
        "Stored features are different! Expected {:?}, found {:?}",
        features, dataset.data().0
    );

    assert_eq!(&targets, dataset.data().1,
        "Stored targets are different! Expected {:?}, found {:?}",
        targets ,dataset.data().1
    );
}

#[test]
fn test_load_parquet() -> Result<(), Box<dyn std::error::Error>> {
    const TEST_FILE: &str = "tests/fixtures/test_data.parquet";

    setup_test_parquet_data(TEST_FILE)?;

    let dataset = Dataset::from_parquet(TEST_FILE)?;
    let (feature_names, target_name) = dataset.names();
    let (features, targets) = dataset.data();

    assert_eq!(feature_names.clone(), ["f1", "f2"].iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        "Loaded data is different! Expected: {:?}, found {:?}",
        vec!["f1", "f2"], feature_names
    );
    assert_eq!(target_name.clone(), "target".to_string(),
        "Loaded target name is different! Expected: {:?}, found {:?}",
        "target", target_name
    );
    assert_eq!(features.clone(), vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
        "Loaded features are different! Expected: {:?}, found: {:?}",
        vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]], features
    );
    assert_eq!(targets.clone(), vec![7.0, 8.0, 9.0],
        "Loaded target values are different! Expected: {:?}, found: {:?}",
        vec![7.0, 8.0, 9.0], targets
    );

    return Ok(());
}
