#![no_std]

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
static mut BUFFER: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const u8 {
    unsafe { BUFFER.as_ptr() }
}

#[no_mangle]
pub extern "C" fn detect_features() -> i32 {
    let mut min_bright = 255;
    let mut eye_idx = 0;
    let mut mouth_idx = 0;
    let mut max_diff = 0;

    unsafe {
        // Deteksi Mata (Cari area paling gelap di 2/3 atas layar)
        for i in (0..(BUFFER.len() * 2 / 3)).step_by(4) {
            let b = (BUFFER[i] as u32 + BUFFER[i+1] as u32 + BUFFER[i+2] as u32) / 3;
            if b < min_bright {
                min_bright = b;
                eye_idx = i / 4;
            }
        }

        // Deteksi Mulut (Cari area kontras tinggi di 1/3 bawah layar)
        for i in ((BUFFER.len() * 2 / 3)..BUFFER.len()).step_by(4) {
            let r = BUFFER[i] as i32;
            let g = BUFFER[i+1] as i32;
            let diff = (r - g).abs(); // Mulut biasanya lebih merah/kontras
            if diff > max_diff {
                max_diff = diff;
                mouth_idx = i / 4;
            }
        }
    }
    // Pack koordinat: Eye (bits 0-15), Mouth (bits 16-31)
    ((mouth_idx as i32) << 16) | (eye_idx as i32)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
