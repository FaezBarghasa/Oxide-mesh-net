//! ACL engine error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AclError {
    #[error("Rule compilation failed: {0}")]
    CompilationFailed(String),

    #[error("Invalid rule: {0}")]
    InvalidRule(String),

    #[error("Rule not found: {0}")]
    RuleNotFound(String),

    #[error("Evaluation error: {0}")]
    EvaluationError(String),

    #[error("SIMD not supported: {0}")]
    SimdNotSupported(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AclError>;