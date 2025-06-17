use crate::core::engine_config::EngineConfig;

pub struct EngineContext {
    cfg: EngineConfig,
}

impl EngineContext {
    pub fn new(cfg: EngineConfig) -> Self {
        Self { cfg }
    }
}
