use crc::{Crc, CRC_32_ISO_HDLC};
use js_sys::{Map, Object};
use wasm_bindgen::prelude::*;

use crc_calculator::crc_calculator_adapter::CrcCalculatorAdapter;
use crc_calculator::CrcCalculator;

use crate::date_time_converter::dos_date_time_calculator_adapter::DosDateTimeCalculatorAdapter;
use crate::date_time_converter::SystemTime;
use crate::zip_file::zip_blob_factory::ZipBlobFactoryAdapter;
use crate::zip_file::ZipBlobFactory;

mod crc_calculator;
mod zip_file;
mod date_time_converter;
mod directory_hash_map_generator;

#[wasm_bindgen(module = "/js/create_directory_mapping.js")]
extern "C" {
    fn create_directory_mapping(directory_listing: &Object, folder_prefix: String) -> Map;
}

#[wasm_bindgen(module = "/js/get_system_time.js")]
extern "C" {
    fn get_system_time() -> Box<[u16]>;
}

#[wasm_bindgen]
pub fn generate_zip_binary(zip_contents: Object) -> Box<[u8]> {
    let directory_mapping = create_directory_mapping(&zip_contents, String::from(""));

    let directory_hash_map = directory_hash_map_generator::generate_directory_mapping(directory_mapping);

    let crc_calculator = Box::new(CrcCalculatorAdapter {
        crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
    });

    let browser_time = get_system_time();

    let date_time_retriever = Box::new(DosDateTimeCalculatorAdapter {
        date_time: SystemTime {
            hours: browser_time[0],
            minutes: browser_time[1],
            seconds: browser_time[2],
            day: browser_time[3],
            month: browser_time[4],
            year: browser_time[5],
        }
    });

    let zip_blob_factory = ZipBlobFactoryAdapter {
        crc_calculator,
        date_time_retriever,
    };

    zip_blob_factory.create_zip_binary(directory_hash_map)
}
