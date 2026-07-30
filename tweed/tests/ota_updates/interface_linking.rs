use cucumber::{given, then, when, World};
use tweed::{
    core::error::TweedError,
    execution::{engine::Engine, executor::ExecutionState},
    Value,
};
pub type Result<T> = core::result::Result<T, TweedError>;

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

#[given(expr = "an engine with a max step size of {int}")]
fn step_size(world: &mut TestWorld, max_step_size: usize) {
    world.engine = Engine::new(max_step_size)
}

#[given(expr = "the engine has safe points activated")]
fn safe_points(world: &mut TestWorld) {
    tweed::register_safe_point(&mut world.engine).unwrap();
}
#[given(
    regex = r"^an engine with an (pricing|order) interface implementation registered in version (0.1.0|0.1.1)"
)]
fn adder_interface(world: &mut TestWorld, wasm: String, version: String) {
    let path = format!("tests/ota_updates/fixtures/{}@{}.wasm", wasm, version);
    let adder = std::fs::read(path).unwrap();
    tweed::create_and_bind(&adder, &mut world.engine).unwrap();
}
#[given(
    regex = r"^an engine with an (pricing|order) interface implementation using safe points registered in version (0.1.0|0.1.1)"
)]
fn interface_with_safe_points(world: &mut TestWorld, wasm: String, version: String) {
    let path = format!(
        "tests/ota_updates/fixtures/safe_points_{}@{}.wasm",
        wasm, version
    );
    let adder = std::fs::read(path).unwrap();
    tweed::create_and_bind(&adder, &mut world.engine).unwrap();
}
#[when(expr = "I upgrade the {string} interface for the {string} interface")]
fn upgrade(world: &mut TestWorld, upgrade_iface: String, selection: String) {
    tweed::upgrade_interface(upgrade_iface, vec![selection], &mut world.engine).unwrap();
}
#[when(expr = "I force upgrade the {string} interface for the {string} interface")]
fn force_upgrade(world: &mut TestWorld, upgrade_iface: String, selection: String) {
    tweed::force_upgrade_interface(upgrade_iface, vec![selection], &mut world.engine).unwrap();
}

#[when(expr = "I invoke the {string} function of interface {string} with the argument {int}")]
fn invoke(world: &mut TestWorld, func: String, iface_name: String, a: i32) {
    world.result = tweed::invoke_interface(
        iface_name.to_string(),
        func,
        vec![Value::I32(a)],
        &mut world.engine,
    );
}

#[when(expr = "I run {int} steps")]
fn step(world: &mut TestWorld, steps: u32) {
    for _ in 0..steps {
        world.result = tweed::step(&mut world.engine);
    }
}

#[when("I run the garbage collector")]
fn gc(world: &mut TestWorld) {
    tweed::collect_garbage(&mut world.engine).unwrap();
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
        _ => {
            println!("Result: {:?}", world.result);
            assert!(false)
        }
    }
}
#[then(expr = "the result should be the error: {string}")]
fn result_err(world: &mut TestWorld, msg: String) {
    match &world.result {
        Err(err) => assert_eq!(err.to_string(), msg),
        _ => {
            println!("Result: {:?}", world.result);
            assert!(false)
        }
    }
}
#[then("the execution should be interrupted")]
fn result_continue(world: &mut TestWorld) {
    match &world.result {
        Ok(ExecutionState::Continue) => assert!(true),
        _ => {
            println!("Result: {:?}", world.result);
            assert!(false)
        }
    }
}
#[then("the execution should be stopped")]
fn result_stopped(world: &mut TestWorld) {
    match &world.result {
        Ok(ExecutionState::Stopped) => assert!(true),
        _ => {
            println!("Result: {:?}", world.result);
            assert!(false)
        }
    }
}

fn main() {
    futures::executor::block_on(TestWorld::run("tests/ota_updates/features/linking.feature"));
    futures::executor::block_on(TestWorld::run(
        "tests/ota_updates/features/interrupting.feature",
    ));
}
