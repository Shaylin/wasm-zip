use std::collections::HashMap;
use std::iter::Zip;
use js_sys::{Map, Uint8Array};
use wasm_bindgen::JsValue;
use crate::CrcCalculator;
use crate::date_time_retriever::DosDateTimeRetriever;
use crate::zip_file::zip_file_entry::ZipFileEntry;
use crate::zip_file::ZipBlobFactory;

pub struct ZipBlobFactoryAdapter {
    crc_calculator: Box<dyn CrcCalculator>,
    date_time_retriever: DosDateTimeRetriever,
}

//TODO: It seems we cannot easily create a map in rust land - or at the very least, it will be miserable
//TODO: So rather than doing everything on the fly, we need to convert this thing into a nicer rust struct
//TODO: We will certainly be able to test that

impl ZipBlobFactory for ZipBlobFactoryAdapter {
    fn create_zip_blob(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Box<[u8]> {
        todo!()
    }
}

impl ZipBlobFactoryAdapter {
    fn create_zip_file_entries(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Vec<ZipFileEntry> {
        let mut zip_file_entries: Vec<ZipFileEntry> = Vec::new();

        let mut file_header_offset: u32 = 0;

        for (file_name, file_body) in directory_mapping.into_iter() {
            let zip_entry = self.create_zip_file_entry(file_name, file_body, file_header_offset);
            file_header_offset += self.get_zip_file_size(&zip_entry);
            zip_file_entries.push(zip_entry);
        }

        zip_file_entries
    }

    fn create_zip_file_entry(&self, zip_file_name: String, file_body: Vec<u8>, header_offset: u32) -> ZipFileEntry {
        let file_crc = self.calculate_file_crc(&file_body);

        ZipFileEntry {
            body: file_body,
            crc: file_crc,
            file_name: zip_file_name,
            dos_time: self.date_time_retriever.get_current_dos_time(),
            dos_date: self.date_time_retriever.get_current_dos_date(),
            entry_offset: header_offset,
        }
    }

    fn calculate_file_crc(&self, file_contents: &Vec<u8>) -> u32 {
        let contents_byte_slice = &file_contents[..];

        self.crc_calculator.calculate_crc32(contents_byte_slice)
    }

    fn get_zip_file_size(&self, zip_file: &ZipFileEntry) -> u32 {
        let local_file_header_size = zip_file.get_local_file_header_size() as u32;
        let body_size = zip_file.body.len() as u32;

        local_file_header_size + body_size
    }

    fn get_end_of_central_directory_record() -> Vec<u8> {
        let mut end_of_central_directory_record: Vec<u8> = Vec::with_capacity(22);


        end_of_central_directory_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_byte() {}
}