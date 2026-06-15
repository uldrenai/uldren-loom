//! Typed errors with stable, machine-readable codes.
//!
//! Language bindings preserve [`Code`] verbatim so error handling is uniform across languages and
//! transports.

use thiserror::Error;

/// Stable, machine-readable error codes. Discriminants are stable and map to HTTP / JSON-RPC codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Code {
    /// Path, ref, or object does not exist.
    NotFound,
    /// Target already exists where creation expected none.
    AlreadyExists,
    /// An object failed canonical-form validation.
    CorruptObject,
    /// A digest did not match its address (tamper / bit-rot).
    IntegrityFailure,
    /// Provider lacks the required capability.
    Unsupported,
    /// Malformed input.
    InvalidArgument,
    /// Underlying I/O error.
    Io,
    /// Unexpected invariant violation.
    Internal,
}

impl Code {
    /// A stable string form, used in logs and wire encodings.
    pub const fn as_str(self) -> &'static str {
        match self {
            Code::NotFound => "NOT_FOUND",
            Code::AlreadyExists => "ALREADY_EXISTS",
            Code::CorruptObject => "CORRUPT_OBJECT",
            Code::IntegrityFailure => "INTEGRITY_FAILURE",
            Code::Unsupported => "UNSUPPORTED",
            Code::InvalidArgument => "INVALID_ARGUMENT",
            Code::Io => "IO",
            Code::Internal => "INTERNAL",
        }
    }
}

/// The error type returned across the engine: a stable [`Code`] plus a human-readable message.
#[derive(Debug, Clone, Error)]
#[error("{}: {message}", code.as_str())]
pub struct LoomError {
    /// Stable, programmatic error code.
    pub code: Code,
    /// Human-readable detail (never parsed by callers).
    pub message: String,
}

impl LoomError {
    /// Construct an error with an explicit code and message.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Convenience constructor for [`Code::NotFound`].
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(Code::NotFound, message)
    }

    /// Convenience constructor for [`Code::CorruptObject`].
    pub fn corrupt(message: impl Into<String>) -> Self {
        Self::new(Code::CorruptObject, message)
    }

    /// Convenience constructor for [`Code::InvalidArgument`].
    pub fn invalid(message: impl Into<String>) -> Self {
        Self::new(Code::InvalidArgument, message)
    }

    /// Convenience constructor for [`Code::Unsupported`].
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new(Code::Unsupported, message)
    }
}

/// The crate-wide result alias.
pub type Result<T> = std::result::Result<T, LoomError>;
