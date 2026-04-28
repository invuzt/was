#![no_std]
slint::include_modules!();
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_slint() {
    let ui = AppWindow::new().unwrap();
    
    ui.on_take_photo(|| {
        unsafe { js_capture_shutter(); }
    });

    ui.run().unwrap();
}

#[wasm_bindgen]
extern "C" {
    fn js_capture_shutter();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
