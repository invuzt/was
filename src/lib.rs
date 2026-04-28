#![no_std]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
