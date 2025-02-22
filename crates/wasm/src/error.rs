use c2zk_frontend_shared::ModuleBuilderError;
use thiserror::Error;

/// A WebAssembly translation error.
///
/// When a WebAssembly function can't be translated, one of these error codes will be returned
/// to describe the failure.
#[derive(Error, Debug)]
pub enum WasmError {
    /// The input WebAssembly code is invalid.
    ///
    /// This error code is used by a WebAssembly translator when it encounters invalid WebAssembly
    /// code. This should never happen for validated WebAssembly code.
    #[error("Invalid input WebAssembly code at offset {offset}: {message}")]
    InvalidWebAssembly {
        /// A string describing the validation error.
        message: String,
        /// The bytecode offset where the error occurred.
        offset: usize,
    },

    /// A feature used by the WebAssembly code is not supported by the embedding environment.
    ///
    /// Embedding environments may have their own limitations and feature restrictions.
    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    /// Any user-defined error.
    #[error("User error: {0}")]
    User(String),

    /// Error in module builder
    #[error("ModuleBuilderError: {0:?}")]
    ModuleBuilderError(#[from] ModuleBuilderError),
}

/// Return an `Err(WasmError::Unsupported(msg))` where `msg` the string built by calling `format!`
/// on the arguments to this macro.
#[macro_export]
macro_rules! wasm_unsupported {
    ($($arg:tt)*) => { WasmError::Unsupported(format!($($arg)*)) }
}

impl From<wasmparser::BinaryReaderError> for WasmError {
    /// Convert from a `BinaryReaderError` to a `WasmError`.
    fn from(e: wasmparser::BinaryReaderError) -> Self {
        Self::InvalidWebAssembly {
            message: e.message().into(),
            offset: e.offset(),
        }
    }
}
