#![no_std]
use libm::{sinf, cosf, tanf};

// Sediakan static buffer agar pointer-nya tetap (tidak berpindah-pindah)
static mut MATRICES: [f32; 32] = [0.0; 32];

#[no_mangle]
pub extern "C" fn get_mat_ptr() -> *const f32 {
    unsafe { MATRICES.as_ptr() }
}

#[no_mangle]
pub extern "C" fn make_transforms(angle: f32, aspect: f32) {
    let s = sinf(angle);
    let c = cosf(angle);
    unsafe {
        // Rotasi Y (Indices 0-15)
        MATRICES[0] = c;   MATRICES[1] = 0.0; MATRICES[2] = s;   MATRICES[3] = 0.0;
        MATRICES[4] = 0.0; MATRICES[5] = 1.0; MATRICES[6] = 0.0; MATRICES[7] = 0.0;
        MATRICES[8] = -s;  MATRICES[9] = 0.0; MATRICES[10] = c;  MATRICES[11] = 0.0;
        MATRICES[12] = 0.0;MATRICES[13] = 0.0;MATRICES[14] = -4.0;MATRICES[15] = 1.0;

        // Perspektif (Indices 16-31)
        let f = 1.0 / tanf(0.4); // Sekitar 45 derajat
        MATRICES[16] = f / aspect;
        MATRICES[21] = f;
        MATRICES[26] = -1.0;
        MATRICES[27] = -1.0;
        MATRICES[30] = -0.2;
        MATRICES[31] = 0.0;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
