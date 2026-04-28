#![no_std]

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
static mut BUFFER: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const u8 {
    unsafe { BUFFER.as_ptr() }
}

#[no_mangle]
pub extern "C" fn detect_brightest() -> i32 {
    let mut brightest_val = 0;
    let mut brightest_idx = 0;
    unsafe {
        for i in (0..BUFFER.len()).step_by(4) {
            let r = BUFFER[i] as u32;
            let g = BUFFER[i+1] as u32;
            let b = BUFFER[i+2] as u32;
            let brightness = (r + g + b) / 3;
            if brightness > brightest_val {
                brightest_val = brightness;
                brightest_idx = i / 4;
            }
        }
    }
    brightest_idx as i32
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
