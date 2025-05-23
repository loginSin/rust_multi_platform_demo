use crate::core::engine_builder_param::EngineBuilderParam;
use crate::ffi::enums::ffi_engine_error::FfiEngineError;
use crate::ffi::enums::ffi_platform_type::FfiPlatform;
use crate::ffi::ffi_helper::ToCow;
use std::ffi::c_char;

#[repr(C)]
pub struct FfiEngineBuilderParam {
    pub key: *const c_char,
    pub path: *const c_char,
    pub platform: FfiPlatform,
}

impl Default for FfiEngineBuilderParam {
    fn default() -> Self {
        Self {
            key: std::ptr::null(),
            path: std::ptr::null(),
            platform: FfiPlatform::Unknown,
        }
    }
}

impl Drop for FfiEngineBuilderParam {
    fn drop(&mut self) {
        unsafe {
            if !self.key.is_null() {
                let _ = Box::from_raw(self.key as *mut c_char);
            }
            if !self.path.is_null() {
                let _ = Box::from_raw(self.path as *mut c_char);
            }
        }
    }
}

impl TryInto<EngineBuilderParam> for &FfiEngineBuilderParam {
    type Error = FfiEngineError;

    fn try_into(self) -> Result<EngineBuilderParam, Self::Error> {
        if self.key.is_null() {
            return Err(FfiEngineError::InvalidKey);
        }
        if self.path.is_null() {
            return Err(FfiEngineError::InvalidPath);
        }
        if self.platform == FfiPlatform::Unknown {
            return Err(FfiEngineError::InvalidPlatform);
        }
        let param = EngineBuilderParam::new(
            self.key.to_cow().as_ref(),
            self.path.to_cow().as_ref(),
            self.platform.into(),
        );
        Ok(param)
    }
}
