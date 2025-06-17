use crate::core::engine_error::EngineError;
use crate::core::models::user_info::UserInfo;
use crate::ffi::enums::ffi_engine_error::FfiEngineError;
use crate::ffi::ffi_callback_def::{FFiCommonStringCb, FFiUpdateUserInfoCb};
use crate::ffi::ffi_helper::ToCString;
use crate::ffi::models::ffi_user_info::FfiUserInfo;
use std::ffi::c_void;

pub(crate) struct FfiContextPtr(pub *const c_void);

unsafe impl Send for FfiContextPtr {}

unsafe impl Sync for FfiContextPtr {}

impl FfiContextPtr {
    pub(crate) fn common_string_callback(
        &self,
        callback: FFiCommonStringCb,
        ret: Result<String, EngineError>,
    ) {
        if let Some(callback) = callback {
            match ret {
                Ok(info) => {
                    callback(
                        self.0,
                        FfiEngineError::Success,
                        info.to_cstring().into_raw(),
                    );
                }
                Err(error) => {
                    callback(self.0, FfiEngineError::from(error), std::ptr::null());
                }
            }
        }
    }
    pub(crate) fn update_user_info_callback(
        &self,
        callback: FFiUpdateUserInfoCb,
        ret: Result<UserInfo, EngineError>,
    ) {
        if let Some(callback) = callback {
            match ret {
                Ok(info) => {
                    let ffi_info = FfiUserInfo::from_rust(info);
                    callback(
                        self.0,
                        FfiEngineError::Success,
                        &ffi_info as *const FfiUserInfo,
                    );
                }
                Err(error) => {
                    callback(self.0, FfiEngineError::from(error), std::ptr::null());
                }
            }
        }
    }
}
