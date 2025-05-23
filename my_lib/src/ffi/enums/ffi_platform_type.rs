use crate::core::enums::platform_type::Platform;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum FfiPlatform {
    Android = 0,
    IOS = 1,
    HarmonyOS = 2,
    Windows = 3,
    MacOS = 4,
    Linux = 5,
    Electron = 6,
    Web = 7,
    /// 未知平台，新增枚举时在此之前添加
    Unknown = 127,
}

impl From<Platform> for FfiPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Android => Self::Android,
            Platform::IOS => Self::IOS,
            Platform::HarmonyOS => Self::HarmonyOS,
            Platform::Windows => Self::Windows,
            Platform::MacOS => Self::MacOS,
            Platform::Linux => Self::Linux,
            Platform::Electron => Self::Electron,
            Platform::Web => Self::Web,
            Platform::Unknown => Self::Unknown,
        }
    }
}

impl Into<Platform> for FfiPlatform {
    fn into(self) -> Platform {
        match self {
            FfiPlatform::Android => Platform::Android,
            FfiPlatform::IOS => Platform::IOS,
            FfiPlatform::HarmonyOS => Platform::HarmonyOS,
            FfiPlatform::Windows => Platform::Windows,
            FfiPlatform::MacOS => Platform::MacOS,
            FfiPlatform::Linux => Platform::Linux,
            FfiPlatform::Electron => Platform::Electron,
            FfiPlatform::Web => Platform::Web,
            FfiPlatform::Unknown => Platform::Unknown,
        }
    }
}
