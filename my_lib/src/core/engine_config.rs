use crate::core::enums::platform_type::Platform;

#[derive(Default)]
pub struct EngineConfig {
    pub key: String,
    pub path: String,
    pub platform: Platform,
}
