use crate::core::enums::platform_type::Platform;

pub struct EngineBuilderParam {
    key: String,
    path: String,
    platform: Platform,
}

impl EngineBuilderParam {
    pub fn new(key: &str, path: &str, platform: Platform) -> Self {
        Self {
            key: key.to_string(),
            path: path.to_string(),
            platform,
        }
    }
}

impl EngineBuilderParam {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_platform(&self) -> Platform {
        self.platform
    }
}
