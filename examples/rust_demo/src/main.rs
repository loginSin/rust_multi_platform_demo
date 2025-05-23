use my_lib::core::engine::Engine;

fn main() {
    let engine = Engine::default();
    let sum = engine.add_int(1, 3);
    assert_eq!(sum, 4);
}
