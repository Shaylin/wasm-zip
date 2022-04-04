pub mod crc_calculator_adapter;

pub trait CrcCalculator {
    fn calculate_crc32(&self, data: &[u8]) -> u32;
}