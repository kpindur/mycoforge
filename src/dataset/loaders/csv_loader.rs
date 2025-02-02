use std::path::Path;
use csv::ReaderBuilder;

use crate::dataset::error::DatasetError;

pub(crate) fn load_csv(
        path: &str, 
        n_features: usize
    ) -> Result<(Vec<String>, String, Vec<Vec<f64>>, Vec<f64>), DatasetError> {
    let path = Path::new(path);
    if !path.exists() { 
        return Err(DatasetError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File not found! path: {:?}", path))));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("csv") {
        return Err(DatasetError::InvalidFormat("File must be a CSV".into()));
    }

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| DatasetError::IoError(e.into()))?;

    let headers = reader.headers()
        .map_err(|_| DatasetError::InvalidFormat("Cannot read headers".into()))?.iter()
        .map(String::from)
        .collect::<Vec<String>>();

    if headers.is_empty() { return Err(DatasetError::EmptyDataset); }
    if headers.len() <= n_features { return Err(DatasetError::InvalidFormat("Not enough columns!".into())); }

    let (feature_names, target_names) = headers.split_at(n_features);

    if target_names.len() > 1 { return Err(DatasetError::InvalidFormat("Too many target names!".into())); }
    let target_name = target_names[0].clone();

    let mut features: Vec<Vec<f64>> = vec![Vec::new(); n_features];
    let mut targets: Vec<f64> = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| DatasetError::ParseError(e.to_string()))?;

        for (i, field) in record.iter().enumerate() {
            let value = field.parse::<f64>()
                .map_err(|_| DatasetError::ParseError(format!("Invalid number: {}", field)))?;
            if i < n_features {
                features[i].push(value);
            } else {
                targets.push(value); 
            }
        }
    }

    return Ok((
        feature_names.to_vec(),
        target_name,
        features, 
        targets
    ));
}
