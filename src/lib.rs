use crc::{Algorithm, Crc, CRC_32_BZIP2, CRC_32_ISCSI, CRC_32_ISO_HDLC};
use js_sys::{Map, Object, Uint8Array};
use wasm_bindgen::prelude::*;

use crc_calculator::crc_calculator_adapter::CrcCalculatorAdapter;
use crc_calculator::CrcCalculator;

use crate::date_time_retriever::chrono_system_time_retriever::ChronoSystemTimeRetriever;
use crate::date_time_retriever::dos_date_time_retriever_adapter::DosDateTimeRetrieverAdapter;
use crate::date_time_retriever::fake_time_retriever::FakeTimeRetriever;
use crate::utils::set_panic_hook;
use crate::zip_file::zip_blob_factory::ZipBlobFactoryAdapter;
use crate::zip_file::ZipBlobFactory;

mod utils;
mod crc_calculator;
mod zip_file;
mod date_time_retriever;
mod directory_hash_map_generator;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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

    let date_time_retriever = Box::new(DosDateTimeRetrieverAdapter {
        date_time_retriever: Box::new(FakeTimeRetriever {})
    });

    let zip_blob_factory = ZipBlobFactoryAdapter {
        crc_calculator,
        date_time_retriever,
    };

    let blob = zip_blob_factory.create_zip_blob(directory_hash_map);

    blob
}
