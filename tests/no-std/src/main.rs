#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::ptr::null_mut;

use unwind_context::unwind_context_with_fmt;

#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;

struct DummyAllocator;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
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

#[derive(Clone)]
pub struct CustomPanicDetector;

impl unwind_context::PanicDetector for CustomPanicDetector {
    fn is_panicking(&self) -> bool {
        unimplemented!()
    }
}

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
