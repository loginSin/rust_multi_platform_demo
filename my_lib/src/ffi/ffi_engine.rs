use crate::core::engine::Engine;
use crate::core::engine_builder::EngineBuilder;
use crate::ffi::enums::ffi_engine_error::FfiEngineError;
use crate::ffi::ffi_engine_builder_param::FfiEngineBuilderParam;

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
