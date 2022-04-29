use std::collections::HashMap;
use js_sys::{Map, Uint8Array};
use wasm_bindgen::JsValue;

pub fn generate_directory_mapping(directory_mapping: Map) -> HashMap<String, Vec<u8>> {
    let mut file_name_to_bodies = HashMap::new();

    for file_name in directory_mapping.keys() {
        let file_name = file_name.unwrap();
        let file_contents: Vec<u8> = get_file_contents(&directory_mapping, &file_name);
        let file_name_string = file_name.as_string().unwrap();

        file_name_to_bodies.insert(
            file_name_string,
            file_contents,
        );
    }

    file_name_to_bodies
}

fn get_file_contents(directory_mapping: &Map, file_name: &JsValue) -> Vec<u8> {
    let file_js_value = directory_mapping.get(&file_name);

    Uint8Array::new(&file_js_value).to_vec()
}