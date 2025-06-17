use crate::core::engine_context::EngineContext;
use crate::core::engine_error::EngineError;
use crate::core::enums::platform_type::Platform;
use crate::core::network::http_client::{HttpClient, HttpError};
use std::sync::Arc;
use std::time::Duration;

pub async fn get_url(ctx: &Arc<EngineContext>, url: &str) -> Result<String, EngineError> {
    let http_client = HttpClient::new(Platform::MacOS, Duration::from_secs(10));
    let ret: Result<(u16, String), HttpError> = http_client.get(url).await;
    match ret {
        Ok((code, data)) => Ok(data),
        Err(err) => Err(EngineError::from(err)),
    }
}
