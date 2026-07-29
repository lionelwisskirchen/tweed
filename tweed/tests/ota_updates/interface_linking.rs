use cucumber::{given, then, when, World};
use tweed::{
    binary::custom_section::InterfaceId,
    core::error::ExecutionError,
    execution::{engine::Engine, executor::ExecutionState},
    Value,
};
pub type Result<T> = core::result::Result<T, ExecutionError>;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {
    engine: Engine,
    result: Result<ExecutionState>,
}
impl TestWorld {
    fn new() -> Self {
        Self {
            engine: Engine::default(),
            result: Ok(ExecutionState::Continue),
        }
    }
}
#[given(
    regex = r"^an engine with an (pricing|order) interface implementation registered in version (0.1.0|0.1.1)"
)]
fn adder_interface(world: &mut TestWorld, wasm: String, version: String) {
    let path = format!("tests/ota_updates/fixtures/{}@{}.wasm", wasm, version);
    let adder = std::fs::read(path).unwrap();
    tweed::create_and_bind(&adder, &mut world.engine).unwrap();
}

#[when(expr = "I upgrade the {string} interface for the {string} interface")]
fn upgrade(world: &mut TestWorld, upgrade_iface: String, selection: String) {
    let up = InterfaceId::try_from(upgrade_iface.to_string()).unwrap();
    let sel = vec![InterfaceId::try_from(selection.to_string()).unwrap()];
    world.engine.upgrade_interface(&up, sel).unwrap();
}

#[when(expr = "I invoke the {string} function of interface {string} with the argument {int}")]
fn invoke(world: &mut TestWorld, func: String, iface_name: String, a: i32) {
    let result = world.engine.invoke_interface(
        InterfaceId::try_from(iface_name.to_string()).unwrap(),
        func,
        vec![Value::I32(a)],
    );
    world.result = result;
}

#[when("I run the garbage collector")]
fn gc(world: &mut TestWorld) {
    world.engine.collect_garbage().unwrap();
}

#[when(
    regex = r"^I add an (pricing|order) interface implementation registered in version (0.1.0|0.1.1)"
)]
fn add_interface(world: &mut TestWorld, wasm: String, version: String) {
    let path = format!("tests/ota_updates/fixtures/{}@{}.wasm", wasm, version);
    let adder = std::fs::read(path).unwrap();
    tweed::create_and_bind(&adder, &mut world.engine).unwrap();
}

#[then(expr = "the result should be {int}")]
fn result(world: &mut TestWorld, res: i32) {
    match &world.result {
        Ok(ExecutionState::Finished(resul)) => assert_eq!(*resul, vec![Value::I32(res)]),
        _ => assert!(false),
    }
}
#[then(expr = "the result should be the error: {string}")]
fn result_err(world: &mut TestWorld, msg: String) {
    let err = world.result.as_ref().unwrap_err();
    assert_eq!(err.to_string(), msg);
}

fn main() {
    futures::executor::block_on(TestWorld::run("tests/ota_updates/features/linking.feature"));
}
