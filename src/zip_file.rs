use std::collections::HashMap;

mod zip_file_entry;
pub mod zip_blob_factory;

pub trait ZipBlobFactory {
    fn create_zip_blob(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Box<[u8]>;
}