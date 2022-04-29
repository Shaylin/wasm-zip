use js_sys::{Map, Object, Uint8Array};
use wasm_bindgen::prelude::*;

use crc_calculator::crc_calculator_adapter::CrcCalculatorAdapter;
use crc_calculator::CrcCalculator;

mod utils;
mod crc_calculator;
mod zip_file;
mod date_time_retriever;
mod directory_hash_map_generator;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/js/create_directory_mapping.js")]
extern "C" {
    fn create_directory_mapping(directory_listing: &Object, folder_prefix: String) -> Map;
}

#[wasm_bindgen]
pub fn generate_zip_blob(zip_contents: Object) -> Box<[u8]> {
    let directory_mapping = create_directory_mapping(&zip_contents, String::from(""));

    for file_name_result in directory_mapping.keys() {
        let file_name = file_name_result.unwrap();
        let file_contents = directory_mapping.get(&file_name);
        let file_bytes = Uint8Array::new(&file_contents).to_vec();
        let file_vector = file_bytes.to_vec();

        let message = format!("Data {:?}", file_vector);
        alert(&message[..]);
    }

    let ting: &[u8] = &[2, 3, 4, 5, 6, 6, 7, 8];

    Box::from(ting)
}
