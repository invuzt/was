#![no_std]

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
static mut BUFFER: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const u8 {
    unsafe { BUFFER.as_ptr() }
}

#[no_mangle]
pub extern "C" fn detect_geometry() -> i64 {
    let mut e1: (u32, u32, u32) = (0, 0, 0); // x, y, count (Mata Kiri)
    let mut e2: (u32, u32, u32) = (0, 0, 0); // x, y, count (Mata Kanan)
    let mut n: (u32, u32, u32) = (0, 0, 0);  // x, y, count (Hidung)
    let mut m: (u32, u32, u32) = (0, 0, 0);  // x, y, count (Mulut)

    unsafe {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = (y * WIDTH + x) * 4;
                let r = BUFFER[i] as u32;
                let g = BUFFER[i+1] as u32;
                let b = BUFFER[i+2] as u32;
                let bright = (r + g + b) / 3;

                // ZONING & THRESHOLDING
                if y < 45 { // Area Mata (Atas)
                    if bright < 60 { // Titik gelap
                        if x < 50 { e1.0 += x as u32; e1.1 += y as u32; e1.2 += 1; }
                        else { e2.0 += x as u32; e2.1 += y as u32; e2.2 += 1; }
                    }
                } else if y < 70 { // Area Hidung (Tengah)
                    if bright > 180 { // Titik terang (ujung hidung biasanya kena cahaya)
                        n.0 += x as u32; n.1 += y as u32; n.2 += 1;
                    }
                } else { // Area Mulut (Bawah)
                    if (r as i32 - g as i32).abs() > 20 { // Kontras bibir
                        m.0 += x as u32; m.1 += y as u32; m.2 += 1;
                    }
                }
            }
        }
    }

    // Hitung Centroid (Rata-rata posisi)
    let p1 = if e1.2 > 0 { (e1.1 / e1.2) << 8 | (e1.0 / e1.2) } else { 0 };
    let p2 = if e2.2 > 0 { (e2.1 / e2.2) << 8 | (e2.0 / e2.2) } else { 0 };
    let p3 = if n.2 > 0 { (n.1 / n.2) << 8 | (n.0 / n.2) } else { 0 };
    let p4 = if m.2 > 0 { (m.1 / m.2) << 8 | (m.0 / m.2) } else { 0 };

    // Pack 4 koordinat ke 64-bit
    ((p4 as i64) << 48) | ((p3 as i64) << 32) | ((p2 as i64) << 16) | (p1 as i64)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
