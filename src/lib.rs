#![no_std]

use libm::{sinf, cosf, tanf};

#[no_mangle]
pub extern "C" fn make_transforms(angle: f32, aspect: f32, mat_ptr: *mut f32) {
    let s = sinf(angle);
    let c = cosf(angle);
    
    unsafe {
        // --- Matrix Rotasi Y ---
        *mat_ptr.add(0) = c;    *mat_ptr.add(1) = 0.0;  *mat_ptr.add(2) = s;   *mat_ptr.add(3) = 0.0;
        *mat_ptr.add(4) = 0.0;  *mat_ptr.add(5) = 1.0;  *mat_ptr.add(6) = 0.0; *mat_ptr.add(7) = 0.0;
        *mat_ptr.add(8) = -s;   *mat_ptr.add(9) = 0.0;  *mat_ptr.add(10) = c;  *mat_ptr.add(11) = 0.0;
        *mat_ptr.add(12) = 0.0; *mat_ptr.add(13) = 0.0; *mat_ptr.add(14) = -3.0; *mat_ptr.add(15) = 1.0; // Translation Z=-3

        // --- Matrix Perspektif ---
        let fov = 1.0 / tanf(45.0 * 0.5 * 3.14159 / 180.0); // FOV 45 deg
        let near = 0.1;
        let far = 100.0;
        
        *mat_ptr.add(16) = fov / aspect;
        *mat_ptr.add(21) = fov;
        *mat_ptr.add(26) = (far + near) / (near - far);
        *mat_ptr.add(27) = -1.0;
        *mat_ptr.add(30) = (2.0 * far * near) / (near - far);
        *mat_ptr.add(31) = 0.0;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
