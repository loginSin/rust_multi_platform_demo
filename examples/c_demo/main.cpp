#include <iostream>

extern "C" {
#include "ffi_client.h"
}



int main() {
    FfiEngineBuilderParam param;
    param.key = "test_key";
    param.path = "test_path";
    param.platform = FfiPlatform_MacOS;

    FfiEngineBuilder *builder;
    FfiEngineError code = ffi_create_engine_builder(&param, &builder);

    FfiEngine *engine;
    code = ffi_engine_builder_build(builder, &engine);

    int sum = ffi_engine_add_int(engine, 1 ,2);
    printf("1 + 2 = %d\n", sum);

    return 0;
}
