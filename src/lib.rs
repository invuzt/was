#![no_std]

#[no_mangle]
pub extern "C" fn get_rotation_matrix(angle: f32, out_ptr: *mut f32) {
    let s = libm::sinf(angle);
    let c = libm::cosf(angle);
    unsafe {
        // Matrix 4x4 Rotasi Y sederhana
        *out_ptr.add(0) = c;    *out_ptr.add(1) = 0.0;  *out_ptr.add(2) = s;   *out_ptr.add(3) = 0.0;
        *out_ptr.add(4) = 0.0;  *out_ptr.add(5) = 1.0;  *out_ptr.add(6) = 0.0; *out_ptr.add(7) = 0.0;
        *out_ptr.add(8) = -s;   *out_ptr.add(9) = 0.0;  *out_ptr.add(10) = c;  *out_ptr.add(11) = 0.0;
        *out_ptr.add(12) = 0.0; *out_ptr.add(13) = 0.0; *out_ptr.add(14) = 0.0; *out_ptr.add(15) = 1.0;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
