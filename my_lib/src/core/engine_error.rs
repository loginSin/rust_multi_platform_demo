use crate::core::network::http_client::HttpError;

pub enum EngineError {
    Success = 0,
    HttpError,
    InvalidArgumentEngine,
}

impl From<HttpError> for EngineError {
    fn from(value: HttpError) -> Self {
        match value {
            _ => EngineError::HttpError,
        }
    }
}
