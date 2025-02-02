use mycoforge::{common::traits::Data, dataset::core::Dataset};

fn sample_data() -> (Vec<String>, Vec<Vec<f64>>) {
    let headers = vec!["x".to_string(), "y".to_string()];
    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 4.0, 6.0, 8.0, 10.0], 
    ];

    return (headers, data);
}

#[test]
fn test_manual_creation() {
    let (headers, data) = sample_data();

    let features = vec![data[0].clone()];
    let targets = data[1].clone();
    let dataset = Dataset::new(
        headers[0..headers.len()-2].to_vec(), headers[headers.len()-1].clone(),
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

    let dataset = Dataset::from_vector(
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
