#![no_std]

#[no_mangle]
pub extern "C" fn process_frame(val: i32) -> i32 {
    // Contoh logika: jika kita ingin memproses data pixel nantinya
    val + 1 
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
