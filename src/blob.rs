pub mod zip_blob_size_calculator;

pub trait BlobSizeCalculator {
    // TODO: Find the correct data types to pass in for name and file contents
    fn calculate_zip_blob_size() -> u32;
}