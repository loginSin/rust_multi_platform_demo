use crate::ffi::enums::ffi_engine_error::FfiEngineError;
use crate::ffi::models::ffi_user_info::FfiUserInfo;
use std::ffi::{c_char, c_void};

/// C 回调
///
/// 成功时，code 为 Success，user_info 为非空指针，表示更新成功。
/// 失败时，code 为 Error，user_info 为空指针。
pub type FFiUpdateUserInfoCb = Option<
    extern "C" fn(context: *const c_void, code: FfiEngineError, user_info: *const FfiUserInfo),
>;

pub type FFiCommonStringCb =
    Option<extern "C" fn(context: *const c_void, code: FfiEngineError, user_info: *const c_char)>;
