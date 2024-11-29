use std::path::Path;
use csv::ReaderBuilder;

use crate::dataset::error::DatasetError;

pub(crate) fn load_csv(path: &Path) -> Result<(Vec<String>, Vec<Vec<f64>>), DatasetError> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| DatasetError::IoError(e.into()))?;

    let headers = reader.headers()
        .map_err(|_| DatasetError::InvalidFormat("Cannot read headers".into()))?.iter()
        .map(String::from)
        .collect::<Vec<String>>();

    if headers.is_empty() { return Err(DatasetError::EmptyDataset); }

    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); headers.len()];

    for result in reader.records() {
        let record = result.map_err(|e| DatasetError::ParseError(e.to_string()))?;

        for (i, field) in record.iter().enumerate() {
            let value = field.parse::<f64>()
                .map_err(|_| DatasetError::ParseError(format!("Invalid number: {}", field)))?;
            columns[i].push(value);
        }
    }

    return Ok((headers, columns));
}
