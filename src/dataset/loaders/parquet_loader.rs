use std::path::Path;
use std::collections::HashMap;
use arrow::array::Float64Array;
use arrow::array::RecordBatch;
use arrow::array::RecordBatchReader;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::dataset::error::DatasetError;
use crate::dataset::core::OutputData;

fn validate_parquet_path(path: &str) -> Result<(), DatasetError> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(DatasetError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "File does not exist"
        )));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("parquet") {
        return Err(DatasetError::InvalidFormat("File must be a Parquet".into()));
    }
    return Ok(());
}

fn process_batch(
    batch: &RecordBatch,
    features: &mut [Vec<f64>],
    target: &mut Vec<f64>,
    n_features: usize
) -> Result<(), DatasetError> {
    // Process feature columns
    for (i, feature) in features.iter_mut().enumerate() {
        let column = batch.column(i);
        if let Some(array) = column.as_any().downcast_ref::<Float64Array>() {
            feature.extend(array.iter().map(|x| x.unwrap_or(f64::NAN)));
        } else {
            return Err(DatasetError::InvalidFormat(
                format!("Column {} is not f64", i)
            ));
        }
    }

    // Process target column
    let target_column = batch.column(n_features);
    if let Some(array) = target_column.as_any().downcast_ref::<Float64Array>() {
        target.extend(array.iter().map(|x| x.unwrap_or(f64::NAN)));
    } else {
        return Err(DatasetError::InvalidFormat("Target column is not f64".into()));
    }

    return Ok(());
}

pub(crate) fn load_parquet(
    path: &str
) -> Result<OutputData, DatasetError> {
    validate_parquet_path(path)?;

    let file = std::fs::File::open(path)
        .map_err(DatasetError::IoError)?;

    let builder = ParquetRecordBatchReaderBuilder::try_new(file)
        .map_err(|e| DatasetError::ParseError(e.to_string()))?;
    
    let reader = builder.build()
        .map_err(|e| DatasetError::ParseError(e.to_string()))?;

    let schema = reader.schema();
    let fields: Vec<String> = schema.fields()
        .iter()
        .map(|f| f.name().to_string())
        .collect();

    if fields.len() < 2 {
        return Err(DatasetError::InvalidFormat("Need at least one feature and target".into()));
    }

    let n_features = fields.len() - 1;
    let feature_names = fields[..n_features].to_vec();
    let target_name = fields[n_features].clone();

    let mut features: Vec<Vec<f64>> = vec![Vec::new(); n_features];
    let mut target = Vec::new();

    for batch_result in reader {
        let batch = batch_result.map_err(|e| DatasetError::ParseError(e.to_string()))?;
        process_batch(&batch, &mut features, &mut target, n_features)?;
    }

    return Ok((feature_names, target_name, features, target));
}
