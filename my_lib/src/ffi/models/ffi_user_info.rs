use crate::core::models::user_info::UserInfo;
use crate::ffi::ffi_helper::{ToCString, ToCow};
use std::ffi::{c_char, c_int};

#[repr(C)]
pub struct FfiUserInfo {
    id: *const c_char,
    name: *const c_char,
    age: c_int,
}

impl FfiUserInfo {
    pub fn to_rust(&self) -> UserInfo {
        UserInfo::new(
            self.id.to_cow().to_string().as_str(),
            self.name.to_cow().to_string().as_str(),
            self.age,
        )
    }

    pub fn from_rust(user_info: UserInfo) -> Self {
        FfiUserInfo {
            id: user_info.get_id().to_cstring().into_raw(),
            name: user_info.get_name().to_cstring().into_raw(),
            age: user_info.get_age(),
        }
    }
}
