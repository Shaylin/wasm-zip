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

#[wasm_bindgen]
pub fn greet() {
    let crc_calculator = BitwiseCrcCalculator {};
    let message = format!("CRC {}", crc_calculator.calculate_crc32());
    alert(&message[..]);
}
