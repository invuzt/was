#![no_std]
extern crate alloc;

slint::include_modules!();

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_slint() {
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_change_model(|m| {
        // Kirim event ke JS untuk ganti model
        unsafe { web_change_model(m.as_str().as_ptr(), m.len()); }
    });

    ui.on_toggle_camera(|| {
        unsafe { web_toggle_camera(); }
    });

    ui.run().unwrap();
}

#[wasm_bindgen]
extern "C" {
    fn web_change_model(name_ptr: *const u8, len: usize);
    fn web_toggle_camera();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
