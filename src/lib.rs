#![no_std]

#[no_mangle]
pub extern "C" fn get_face_z(z1: f32, z2: f32, z3: f32, z4: f32) -> f32 {
    (z1 + z2 + z3 + z4) / 4.0
}

#[no_mangle]
pub extern "C" fn calculate_shading(z_depth: f32) -> i32 {
    // Makin jauh (z besar), makin gelap (nilai 0-255)
    let shade = 200.0 - (z_depth * 40.0);
    if shade < 20.0 { 20 } else if shade > 255.0 { 255 } else { shade as i32 }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
