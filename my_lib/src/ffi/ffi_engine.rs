use crate::core::engine::Engine;
use crate::core::engine_builder::EngineBuilder;
use crate::core::engine_error::EngineError;
use crate::ffi::enums::ffi_engine_error::FfiEngineError;
use crate::ffi::ffi_callback_def::{FFiCommonStringCb, FFiUpdateUserInfoCb};
use crate::ffi::ffi_context_ptr::FfiContextPtr;
use crate::ffi::ffi_engine_builder_param::FfiEngineBuilderParam;
use crate::ffi::ffi_helper::{ref_engine, ToCString, ToCow};
use crate::ffi::models::ffi_user_info::FfiUserInfo;
use lazy_static::lazy_static;
use std::ffi::{c_char, c_double, c_int, c_void};
use threadpool::ThreadPool;

/// C 回调线程池，最大线程数
const CB_THREAD_POOL_MAX_NUM: usize = 3;

lazy_static! {
    /// C 回调线程池，直接从 ffi_c 中回调必须在此线程池中执行，如遇到空指针、枚举越界等异常时的回调。
    /// 如果不切线程执行回调，可能会导致调用方死锁，如：
    /// ```cpp
    /// void ConnectCb_Invalid(const void *context, enum EngineError code, const char *) {
    ///     std::unique_lock<std::mutex> lock(tester->mtx_conn_);
    ///     ...
    /// }
    ///
    /// std::unique_lock<std::mutex> lock(TESTER_A.mtx_conn_);
    /// engine_connect(NULL, TESTER_A.token_.c_str(), 5, &TESTER_A, ConnectCb_Invalid);
    /// TESTER_A.cv_conn_.wait(lock);
    /// ```
    static ref CB_POOL: ThreadPool = ThreadPool::new(CB_THREAD_POOL_MAX_NUM);
}

#[no_mangle]
pub extern "C" fn ffi_create_engine_builder(
    builder_param: *mut FfiEngineBuilderParam,
    out_builder: *mut *mut EngineBuilder,
) -> FfiEngineError {
    if builder_param.is_null() {
        return FfiEngineError::InvalidBuilderParam;
    }
    let ref_param = unsafe { &*builder_param };
    match ref_param.try_into() {
        Ok(builder_param) => {
            let engine_builder = EngineBuilder::new(builder_param);
            unsafe {
                *out_builder = Box::into_raw(Box::new(engine_builder));
            }
            FfiEngineError::Success
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn ffi_engine_builder_build(
    builder: *mut EngineBuilder,
    out_engine: *mut *mut Engine,
) -> FfiEngineError {
    if builder.is_null() {
        return FfiEngineError::InvalidArgumentEngineBuilder;
    }
    let builder = unsafe { Box::from_raw(builder) };
    let engine = builder.build();
    unsafe {
        *out_engine = Box::into_raw(Box::new(engine));
    }
    FfiEngineError::Success
}

#[no_mangle]
pub extern "C" fn ffi_engine_add_int(engine: *const Engine, a: c_int, b: c_int) -> c_int {
    ref_engine(engine).add_int(a, b)
}

#[no_mangle]
pub extern "C" fn ffi_engine_add_string(
    engine: *const Engine,
    a: *const c_char,
    b: *const c_char,
) -> *const c_char {
    let value = ref_engine(engine).add_string(a.to_cow().as_ref(), b.to_cow().as_ref());
    value.to_cstring().as_ptr()
}

#[no_mangle]
pub extern "C" fn ffi_engine_add_double(
    engine: *const Engine,
    a: c_double,
    b: c_double,
) -> c_double {
    ref_engine(engine).add_double(a, b)
}

#[no_mangle]
pub extern "C" fn ffi_engine_add_bool(engine: *const Engine, a: bool, b: bool) -> bool {
    ref_engine(engine).add_bool(a, b)
}

#[no_mangle]
pub extern "C" fn ffi_engine_update_user_info(
    engine: *const Engine,
    user_info: *const FfiUserInfo,
    context: *const c_void,
    callback: FFiUpdateUserInfoCb,
) {
    let context_ptr = FfiContextPtr(context);
    if engine.is_null() {
        return CB_POOL.execute(move || {
            context_ptr
                .update_user_info_callback(callback, Err(EngineError::InvalidArgumentEngine));
        });
    }
    let ref_user_info = unsafe { &*user_info };
    let user_info = ref_user_info.to_rust();
    ref_engine(engine).update_user_info(user_info, move |ret| {
        return CB_POOL.execute(move || {
            context_ptr.update_user_info_callback(callback, ret);
        });
    });
}

#[no_mangle]
pub extern "C" fn ffi_engine_get_url(
    engine: *const Engine,
    url: *const c_char,
    context: *const c_void,
    callback: FFiCommonStringCb,
) {
    let context_ptr = FfiContextPtr(context);
    if engine.is_null() {
        return CB_POOL.execute(move || {
            context_ptr.common_string_callback(callback, Err(EngineError::InvalidArgumentEngine));
        });
    }
    ref_engine(engine).get_url(url.to_cow().as_ref(), move |ret| {
        return CB_POOL.execute(move || {
            context_ptr.common_string_callback(callback, ret);
        });
    });
}
