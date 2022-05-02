#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use js_sys::{Object, Reflect};
use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;
use doggy_bag::generate_zip_blob;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn single_file() {
    let directory_object = Object::new();

    let file_name = JsValue::from("BugCat.txt");
    let file_data = JsValue::from("Hello!");

    Reflect::set(&directory_object, &file_name, &file_data).unwrap();

    generate_zip_blob(directory_object);
}

#[wasm_bindgen_test]
fn multiple_files() {
    let directory_object = Object::new();

    let first_file_name = JsValue::from("BugCat.txt");
    let first_file_data = JsValue::from("Hello!");
    let second_file_name = JsValue::from("FoamCat.png");
    let second_file_data = JsValue::from("yow");
    let third_file_name = JsValue::from("Thing.json");
    let third_file_data = JsValue::from("{}");

    Reflect::set(&directory_object, &first_file_name, &first_file_data).unwrap();
    Reflect::set(&directory_object, &second_file_name, &second_file_data).unwrap();
    Reflect::set(&directory_object, &third_file_name, &third_file_data).unwrap();

    generate_zip_blob(directory_object);
}

#[wasm_bindgen_test]
fn multiple_files_with_folders() {
    let directory_object = Object::new();

    let first_file_name = JsValue::from("MyFolder/BugCat.txt");
    let first_file_data = JsValue::from("Hello!");
    let second_file_name = JsValue::from("FoamCat.png");
    let second_file_data = JsValue::from("yow");
    let third_file_name = JsValue::from("AnotherFolder/ExtraFolder/Thing.json");
    let third_file_data = JsValue::from("{}");

    Reflect::set(&directory_object, &first_file_name, &first_file_data).unwrap();
    Reflect::set(&directory_object, &second_file_name, &second_file_data).unwrap();
    Reflect::set(&directory_object, &third_file_name, &third_file_data).unwrap();

    generate_zip_blob(directory_object);
}