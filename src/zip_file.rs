use js_sys::Map;

mod zip_file_entry;
pub mod zip_blob_factory;

pub trait ZipBlobFactory {
    fn create_zip_blob(directory_mapping: Map) -> Box<[u8]>;
}

// TODO: Decide on return types in terms of copying data vs transferring ownership etc
// The header will be a transferring of ownership because its made up of various variables
// The body may have to be a copy anyway