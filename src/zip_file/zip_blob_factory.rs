use js_sys::Map;
use crate::CrcCalculator;
use crate::date_time_retriever::DosDateTimeRetriever;
use crate::zip_file::zip_file_entry::ZipFileEntry;
use crate::zip_file::ZipBlobFactory;

pub struct ZipBlobFactoryAdapter {
    crc_calculator: Box<dyn CrcCalculator>,
    date_time_retriever: DosDateTimeRetriever,
}

impl ZipBlobFactory for ZipBlobFactoryAdapter {
    fn create_zip_blob(directory_mapping: Map) -> Box<[u8]> {
        todo!()
    }
}

impl ZipBlobFactoryAdapter {
    fn create_zip_file_entries(&self, directory_mapping: Map) -> Vec<ZipFileEntry> {
        let mut zip_file_entries: Vec<ZipFileEntry> = Vec::new();

        for file_name in directory_mapping.keys() {
            let file_name = file_name.unwrap();
        }

        zip_file_entries
    }

    fn get_end_of_central_directory_record() -> Vec<u8> {
        let mut end_of_central_directory_record: Vec<u8> = Vec::with_capacity(22);


        end_of_central_directory_record
    }
}