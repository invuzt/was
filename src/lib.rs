#![no_std]

#[no_mangle]
pub extern "C" fn rotate_3d(x: f32, y: f32, z: f32, angle: f32, axis: i32) -> f32 {
    // Sederhana: Sin/Cos Approximation (biar tetap no_std)
    let s = angle.sin();
    let c = angle.cos();
    
    match axis {
        0 => x, // Axis X
        1 => y * c - z * s, // Axis Y
        2 => y * s + z * c, // Axis Z
        _ => 0.0
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
