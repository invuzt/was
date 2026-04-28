#![no_std]

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
static mut BUFFER: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];
static mut LAST_X: f32 = 0.0;
static mut LAST_Y: f32 = 0.0;

#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const u8 {
    unsafe { BUFFER.as_ptr() }
}

#[no_mangle]
pub extern "C" fn detect_focus() -> i32 {
    let mut sum_x: u32 = 0;
    let mut sum_y: u32 = 0;
    let mut count: u32 = 0;

    unsafe {
        for i in (0..BUFFER.len()).step_by(4) {
            let r = BUFFER[i] as u32;
            let g = BUFFER[i+1] as u32;
            let b = BUFFER[i+2] as u32;
            let brightness = (r + g + b) / 3;

            // Hanya proses pixel yang sangat terang (thresholding)
            if brightness > 200 {
                let idx = i / 4;
                sum_x += (idx % WIDTH) as u32;
                sum_y += (idx / WIDTH) as u32;
                count += 1;
            }
        }

        if count > 0 {
            let target_x = (sum_x / count) as f32;
            let target_y = (sum_y / count) as f32;
            
            // Smoothing: 80% posisi lama, 20% posisi baru
            LAST_X = (LAST_X * 0.8) + (target_x * 0.2);
            LAST_Y = (LAST_Y * 0.8) + (target_y * 0.2);
        }

        ((LAST_Y as i32) << 16) | (LAST_X as i32)
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
