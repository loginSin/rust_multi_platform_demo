use crate::core::engine::Engine;
use crate::core::engine_builder_param::EngineBuilderParam;
use crate::core::engine_config::EngineConfig;

pub struct EngineBuilder {
    builder_param: EngineBuilderParam,
}

impl EngineBuilder {
    pub fn new(builder_param: EngineBuilderParam) -> Self {
        Self { builder_param }
    }
}

impl EngineBuilder {
    pub fn build(&self) -> Engine {
        Engine::new(self.build_config())
    }

    fn build_config(&self) -> EngineConfig {
        EngineConfig {
            key: self.builder_param.get_key().to_string(),
            path: self.builder_param.get_path().to_string(),
            platform: self.builder_param.get_platform(),
        }
    }
}
