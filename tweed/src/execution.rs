pub mod adresses;
pub mod constant_expression_executor;
pub mod engine;

#[cfg(not(feature = "tail-call"))]
pub mod exec_instructions;

#[cfg(feature = "tail-call")]
pub mod exec_instructions_tail_call;

pub mod execution_stack;
pub mod executor;
pub mod instances;
pub mod instantiator;
pub mod linker;
pub mod store;
