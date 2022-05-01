use std::alloc::System;
use crc::{Crc, CRC_32_ISO_HDLC};
use js_sys::{Map, Object, Uint8Array};
use wasm_bindgen::prelude::*;

use crc_calculator::crc_calculator_adapter::CrcCalculatorAdapter;
use crc_calculator::CrcCalculator;

use crate::date_time_converter::dos_date_time_calculator_adapter::DosDateTimeCalculatorAdapter;
use crate::date_time_converter::SystemTime;
use crate::utils::set_panic_hook;
use crate::zip_file::zip_blob_factory::ZipBlobFactoryAdapter;
use crate::zip_file::ZipBlobFactory;

mod utils;
mod crc_calculator;
mod zip_file;
mod date_time_converter;
mod directory_hash_map_generator;

#[wasm_bindgen(module = "/js/create_directory_mapping.js")]
extern "C" {
    fn create_directory_mapping(directory_listing: &Object, folder_prefix: String) -> Map;
}

#[wasm_bindgen]
pub fn generate_zip_blob(zip_contents: Object) -> Box<[u8]> {
    set_panic_hook();

    let directory_mapping = create_directory_mapping(&zip_contents, String::from(""));

    let directory_hash_map = directory_hash_map_generator::generate_directory_mapping(directory_mapping);

    //TODO: CRC Is Correct -> Now We Face A Headings Error - End Of Data Found
    let crc_calculator = Box::new(CrcCalculatorAdapter {
        crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
    });

    //TODO: Remove Chrono Dependency - Need To Grab Time From Javascript
    let date_time_retriever = Box::new(DosDateTimeCalculatorAdapter {
        date_time: SystemTime {
            hours: 0,
            minutes: 0,
            seconds: 0,
            day: 0,
            month: 0,
            year: 0,
        }
    });

    let zip_blob_factory = ZipBlobFactoryAdapter {
        crc_calculator,
        date_time_retriever,
    };

    zip_blob_factory.create_zip_blob(directory_hash_map)
}
