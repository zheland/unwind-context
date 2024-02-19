#![no_std]
#![no_main]

use core::fmt::{Result as FmtResult, Write as FmtWrite};
use core::panic::PanicInfo;

use unwind_context::unwind_context_with_fmt;

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
    let _ctx = unwind_context_with_fmt!(
        (fn(a, b, c)),
        writer = Writer,
        panic_detector = CustomPanicDetector,
        color_scheme = None,
    );
    panic!();
}

#[derive(Clone)]
pub struct Writer;

impl FmtWrite for Writer {
    fn write_str(&mut self, _: &str) -> FmtResult {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct CustomPanicDetector;

impl unwind_context::PanicDetector for CustomPanicDetector {
    fn is_panicking(&self) -> bool {
        unimplemented!()
    }
}
