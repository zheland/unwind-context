#![no_std]
#![no_main]
#![allow(
    missing_docs,
    clippy::panic,
    clippy::missing_panics_doc,
    clippy::unimplemented
)]

extern crate alloc;

use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

use unwind_context::unwind_context_with_fmt;

#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;

#[derive(Copy, Clone, Debug)]
struct DummyAllocator;

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn func(a: u32, b: *const u8, c: bool) {
    let mut writer = String::new();
    let _ctx = unwind_context_with_fmt!(
        (fn(a, b, c)),
        writer = &mut writer,
        panic_detector = CustomPanicDetector,
        color_scheme = None,
    );
    panic!();
}

#[derive(Copy, Clone, Debug)]
pub struct CustomPanicDetector;

impl unwind_context::PanicDetector for CustomPanicDetector {
    fn is_panicking(&self) -> bool {
        unimplemented!()
    }
}

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unimplemented!()
    }
}
