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
