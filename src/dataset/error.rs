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
