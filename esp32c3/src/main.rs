#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;
use inplace_wasm_interpreter;
use inplace_wasm_interpreter::Value;
extern crate alloc;
use alloc::vec;
use core::mem::MaybeUninit;
use crate::alloc::string::ToString;
fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    esp_println::logger::init_logger_from_env();
    let byte_code = include_bytes!("sum_example.wasm");
    let result = inplace_wasm_interpreter::run(byte_code, "sum".to_string(), vec![Value::I32(6)]);
    log::info!("Result: {:?}", result);

    loop {}
}
