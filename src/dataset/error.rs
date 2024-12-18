//! Error types for dataset handling.
//!
//! This modules provides error types for handling various dataset-related errors, from file
//! operations to data validation.

/// Errors that can occur during dataset operations.
///
/// # Variants
/// * `FileNotFound(String)` - Specified file path not found
/// * `InvalidFormat(String)` - Dataset format is invalid
/// * `MissingColumn(String)` - Required column is missing
/// * `ParseError(String)` - Failed to parse data value
/// * `EmptyDataset` - Dataset contains no data
/// * `DimensionMismatch` - Number of dimensions doesn't match expected
/// * `IoError(std::io::Error)` - IO operation failed
#[derive(Debug)]
pub enum DatasetError {
    FileNotFound(String),
    InvalidFormat(String),
    MissingColumn(String),
    ParseError(String),
    EmptyDataset,
    DimensionMismatch { expected: usize, found: usize },
    IoError(std::io::Error)
}

impl std::error::Error for DatasetError {}

impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FileNotFound(path) => write!(f, "File not found: {}", path),
            Self::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Self::MissingColumn(col) => write!(f, "Missing column: {}", col),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::EmptyDataset => write!(f, "Dataset is empty"),
            Self::DimensionMismatch { expected, found } => write!(f, "Dimensions do not match: expected {}, found {}", expected, found),
            Self::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::io::Error> for DatasetError {
    fn from(err: std::io::Error) -> Self {
        return Self::IoError(err);
    }
}
