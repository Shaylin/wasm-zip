mod utils;
mod crc;

use wasm_bindgen::prelude::*;
use crc::bitwise_crc_calculator::BitwiseCrcCalculator;
use crc::CrcCalculator;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

//TODO: Return a boxed byte slice - cannot return rust allocated mem
// have to use this box because it needs to copy by value - generates a shim to create the buffer on the js side
#[wasm_bindgen]
pub fn generate_zip_blob() -> Box<[u8]> {
    let crc_calculator = BitwiseCrcCalculator {};
    let message = format!("CRC {}", crc_calculator.calculate_crc32());
    alert(&message[..]);

    let ting: &[u8] = &[2, 3, 4, 5, 6, 6, 7, 8];

    return Box::from(ting);
}
