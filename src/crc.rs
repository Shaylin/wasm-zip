pub mod bitwise_crc_calculator;

pub trait CrcCalculator {
    fn calculate_crc32(&self) -> u32;
}