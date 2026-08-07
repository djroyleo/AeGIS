//! The common error type shared across the `aegis-*` crates.

use std::fmt;
use std::path::PathBuf;

/// Errors that can occur anywhere in the `AeGIS` ecosystem.
///
/// Kept deliberately coarse for now; variants grow as the IO and
/// geoprocessing subsystems gain real implementations.
#[derive(Debug)]
#[non_exhaustive]
pub enum AegisError {
    /// An underlying filesystem or stream error.
    Io(std::io::Error),
    /// No registered driver understands this dataset.
    UnsupportedFormat { path: PathBuf },
    /// A dataset was syntactically readable but semantically invalid.
    InvalidData { message: String },
    /// A geoprocessing tool was invoked with a missing or ill-typed parameter.
    InvalidParameter { name: String, message: String },
    /// Functionality that is designed but not yet implemented.
    NotYetImplemented { feature: &'static str },
}

impl fmt::Display for AegisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::UnsupportedFormat { path } => {
                write!(f, "no driver can open {}", path.display())
            }
            Self::InvalidData { message } => write!(f, "invalid data: {message}"),
            Self::InvalidParameter { name, message } => {
                write!(f, "invalid parameter {name:?}: {message}")
            }
            Self::NotYetImplemented { feature } => {
                write!(f, "not yet implemented: {feature}")
            }
        }
    }
}

impl std::error::Error for AegisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AegisError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
