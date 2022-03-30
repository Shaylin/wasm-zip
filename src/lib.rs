mod utils;
mod crc;

use wasm_bindgen::prelude::*;
use crc::bitwise_crc_calculator::BitwiseCrcCalculator;
use crc::CrcCalculator;
use js_sys::Object;
use js_sys::Map;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/js/create_directory_mapping.js")]
extern "C" {
    fn create_directory_mapping(directory_listing: Object, folder_prefix: String) -> Map;
}

//TODO: Return a boxed byte slice - cannot return rust allocated mem
// have to use this box because it needs to copy by value - generates a shim to create the buffer on the js side
#[wasm_bindgen]
pub fn generate_zip_blob(zip_contents: Object) -> Box<[u8]> {
    let crc_calculator = BitwiseCrcCalculator {};
    let message = format!("CRC {:?}", Object::entries(&zip_contents));
    alert(&message[..]);

    let ting: &[u8] = &[2, 3, 4, 5, 6, 6, 7, 8];

    return Box::from(ting);
}
