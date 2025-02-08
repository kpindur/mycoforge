use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use csv::ReaderBuilder;
use std::collections::HashMap;

use crate::dataset::error::DatasetError;
use crate::dataset::core::{OutputData, Metadata};

fn validate_csv_path(path: &str) -> Result<(), DatasetError> {
    let path = Path::new(path);
    if !path.exists() { 
        return Err(DatasetError::IoError(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found! path: {:?}", path)
            )
        ));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("csv") {
        return Err(DatasetError::InvalidFormat("File must be a CSV".into()));
    }
    return Ok(());
}

fn parse_csv<R: std::io::Read>(
    reader: R,
    n_features: usize
) -> Result<OutputData, DatasetError> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

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

pub(crate) fn load_csv(
        path: &str, 
        n_features: usize
    ) -> Result<OutputData, DatasetError> {
    validate_csv_path(path)?;
    
    let file = File::open(path)
        .map_err(DatasetError::IoError)?;
        
    return parse_csv(file, n_features);
}

pub(crate) fn load_csv_with_metadata(
    path: &str,
    n_features: usize
) -> Result<(Metadata, OutputData), DatasetError> {
    validate_csv_path(path)?;
    
    let file = File::open(path)
        .map_err(DatasetError::IoError)?;
    let reader = BufReader::new(file);

    let mut metadata = HashMap::new();
    let mut data_lines = Vec::new();
    
    for line in reader.lines() {
        let line = line.map_err(DatasetError::IoError)?;
        if line.starts_with('#') {
            if let Some((key, value)) = line.strip_prefix('#')
                .expect("Failed to strip prefix!")
                    .split_once(':') 
            {
                metadata.insert(
                    key.trim().to_string(),
                    value.trim().to_string()
                );
            }
        } else {
            data_lines.push(line);
        }
    }
    
    let csv_content = data_lines.join("\n");
    let output_data = parse_csv(csv_content.as_bytes(), n_features)?;
    
    return Ok((metadata, output_data));
}

