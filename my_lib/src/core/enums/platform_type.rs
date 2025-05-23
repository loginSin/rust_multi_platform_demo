#[derive(Clone, Copy, PartialEq, Default)]
pub enum Platform {
    Android = 0,
    IOS = 1,
    HarmonyOS = 2,
    Windows = 3,
    MacOS = 4,
    Linux = 5,
    Electron = 6,
    Web = 7,
    /// 未知平台，新增枚举时在此之前添加
    #[default]
    Unknown = 127,
}
