use js_sys::{Map, Object};
use wasm_bindgen::prelude::*;

use crc_calculator::crc_calculator_adapter::CrcCalculatorAdapter;
use crc_calculator::CrcCalculator;

mod utils;
mod crc_calculator;
mod blob;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/js/create_directory_mapping.js")]
extern "C" {
    fn create_directory_mapping(directory_listing: &Object, folder_prefix: String) -> Map;
}

//TODO: Return a boxed byte slice - cannot return rust allocated mem
// have to use this box because it needs to copy by value - generates a shim to create the buffer on the js side
#[wasm_bindgen]
pub fn generate_zip_blob(zip_contents: Object) -> Box<[u8]> {
    let directory_mapping = create_directory_mapping(&zip_contents, String::from(""));

    // Maybe unwrap all keys and values and then give it to the blob size calculator to some stuff

    // 1. Determine the size of the zip that's going to be created in bytes
    // 2. Allocate the boxed slice for it on the heap+
    // 3. Start writing headers and files

    let crc_calculator = CrcCalculatorAdapter::new();
    let message = format!("CRC {:?}", directory_mapping.get(&JsValue::from_str("capoo")));
    alert(&message[..]);

    let ting: &[u8] = &[2, 3, 4, 5, 6, 6, 7, 8];

    return Box::from(ting);
}
