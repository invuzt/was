#![no_std]

#[no_mangle]
pub extern "C" fn check_wasm(val: i32) -> i32 {
    val + 1
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
