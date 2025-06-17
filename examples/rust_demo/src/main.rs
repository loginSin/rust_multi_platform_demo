use my_lib::core::engine_builder::EngineBuilder;
use my_lib::core::engine_builder_param::EngineBuilderParam;
use my_lib::core::enums::platform_type::Platform;

fn main() {
    let key = "test_key";
    let path = "test_path";
    let platform = Platform::Android;
    let builder_param = EngineBuilderParam::new(key, path, platform);
    let builder = EngineBuilder::new(builder_param);
    let engine = builder.build();
    let sum = engine.add_int(1, 3);
    assert_eq!(sum, 4);
}
