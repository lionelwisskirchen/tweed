#![no_std]
#![feature(c_size_t)]

extern crate alloc;

use alloc::ffi::CString;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use ::core::alloc::GlobalAlloc;
use ::core::ffi;
use ::core::panic::PanicInfo;
use ::core::ptr::null;
use tweed::*;
use spin::mutex::Mutex;

struct CAllocator(Mutex<()>);

extern "C" {
    pub fn malloc(size: ffi::c_size_t) -> *mut ffi::c_void;
    pub fn free(p: *mut ffi::c_void);
    pub fn realloc(p: *mut ffi::c_void, size: ffi::c_size_t) -> *mut ffi::c_void;
}

unsafe impl GlobalAlloc for CAllocator {
    unsafe fn alloc(&self, layout: ::core::alloc::Layout) -> *mut u8 {
        let guard = self.0.lock();
        let size = layout.size();
        let result = malloc(size) as *mut u8;
        drop(guard);
        result
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: ::core::alloc::Layout) {
        let guard = self.0.lock();
        free(ptr as *mut ffi::c_void);
        drop(guard)
    }
    unsafe fn realloc(&self, ptr: *mut u8, _: ::core::alloc::Layout, new_size: usize) -> *mut u8 {
        let guard = self.0.lock();
        let result = realloc(ptr as *mut ffi::c_void, new_size) as *mut u8;
        drop(guard);
        result
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CAllocator = CAllocator(Mutex::new(()));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn wasm_interp(
    input: *const ffi::c_uchar,
    len: ffi::c_size_t,
) -> *const ffi::c_char {
    let result: Result<Vec<Value>, tweed::core::error::TweedError> = (|| {
        unsafe {
	    tweed::run(::core::slice::from_raw_parts(input, len),"_run".to_string(), vec![])		
        }

    })();
    if result.is_ok() {
        null()
    } else {
        result
            .map_err(|e| CString::new(e.to_string()).unwrap().into_raw())
            .map(|_| null::<ffi::c_char>())
            .unwrap_err()
    }
}

