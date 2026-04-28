#![no_std]
use libm::{sinf, cosf, tanf};

static mut MATRICES: [f32; 32] = [0.0; 32];
static mut CUR_ROT: [f32; 2] = [0.0; 0.0];

#[no_mangle]
pub extern "C" fn get_mat_ptr() -> *const f32 { unsafe { MATRICES.as_ptr() } }

#[no_mangle]
pub extern "C" fn update_params(rx: f32, ry: f32, aspect: f32) {
    unsafe {
        CUR_ROT[0] = rx;
        CUR_ROT[1] = ry;
        
        let sx = sinf(rx); let cx = cosf(rx);
        let sy = sinf(ry); let cy = cosf(ry);

        // Matrix Rotasi Gabungan (X & Y)
        MATRICES[0] = cy;       MATRICES[1] = sy * sx;  MATRICES[2] = sy * cx;  MATRICES[3] = 0.0;
        MATRICES[4] = 0.0;      MATRICES[5] = cx;       MATRICES[6] = -sx;      MATRICES[7] = 0.0;
        MATRICES[8] = -sy;      MATRICES[9] = cy * sx;  MATRICES[10] = cy * cx; MATRICES[11] = 0.0;
        MATRICES[12] = 0.0;     MATRICES[13] = 0.0;     MATRICES[14] = -5.0;    MATRICES[15] = 1.0;

        // Perspektif
        let f = 1.0 / tanf(0.5);
        MATRICES[16] = f / aspect;
        MATRICES[21] = f;
        MATRICES[26] = -1.1;
        MATRICES[27] = -1.0;
        MATRICES[30] = -0.2;
        MATRICES[31] = 0.0;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
