#![no_std]

#[no_mangle]
pub extern "C" fn rotate_x(y: f32, z: f32, angle: f32) -> f32 {
    let s = libm::sinf(angle);
    let c = libm::cosf(angle);
    y * c - z * s
}

#[no_mangle]
pub extern "C" fn rotate_y(x: f32, z: f32, angle: f32) -> f32 {
    let s = libm::sinf(angle);
    let c = libm::cosf(angle);
    x * c + z * s
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
