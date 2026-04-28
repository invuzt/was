#![no_std]
static mut ANGLE_X: f32 = 0.0;
static mut ANGLE_Y: f32 = 0.0;

#[no_mangle]
pub extern "C" fn update_rotation(dx: f32, dy: f32) {
    unsafe { ANGLE_Y += dx * 0.01; ANGLE_X += dy * 0.01; }
}
#[no_mangle]
pub extern "C" fn get_angle_x() -> f32 { unsafe { ANGLE_X } }
#[no_mangle]
pub extern "C" fn get_angle_y() -> f32 { unsafe { ANGLE_Y } }
#[no_mangle]
pub extern "C" fn get_z_avg(z1: f32, z2: f32, z3: f32) -> f32 { (z1 + z2 + z3) / 3.0 }
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
