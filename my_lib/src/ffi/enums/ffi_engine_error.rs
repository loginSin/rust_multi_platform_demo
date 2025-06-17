use crate::core::engine_error::EngineError;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum FfiEngineError {
    Success = 0,
    Error = 1,
    InvalidBuilderParam = 2,
    InvalidKey = 3,
    InvalidPath = 4,
    InvalidPlatform = 5,
    InvalidArgumentEngineBuilder = 6,
}

impl From<EngineError> for FfiEngineError {
    fn from(value: EngineError) -> Self {
        match value {
            EngineError::Success => FfiEngineError::Success,
            EngineError::HttpError => FfiEngineError::Error,
            EngineError::InvalidArgumentEngine => FfiEngineError::Error,
        }
    }
}
