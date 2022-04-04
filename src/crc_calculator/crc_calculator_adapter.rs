use crate::crc_calculator::CrcCalculator;
use crc::{Crc, CRC_32_ISCSI};

pub struct CrcCalculatorAdapter {
    crc: Crc<u32>,
}

impl CrcCalculatorAdapter {
    pub(crate) fn new() -> Self {
        Self {
            crc: Crc::<u32>::new(&CRC_32_ISCSI)
        }
    }
}

//TODO: Figure out if its healthy to keep things like a singleton here - or do we need to init a new crc object every time - that's okay too
impl CrcCalculator for CrcCalculatorAdapter {
    fn calculate_crc32(&self, data: &[u8]) -> u32 {
        let mut digest = self.crc.digest();

        digest.update(data);

        return digest.finalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_byte() {
        let input_byte: &[u8] = &[0x42];

        let crc_calculator = CrcCalculatorAdapter::new();
        assert_eq!(0xF23D3E1A, crc_calculator.calculate_crc32(input_byte));
    }

    #[test]
    fn multiple_bytes() {
        let input_bytes: &[u8] = &[0x42, 0x55, 0x47, 0x43, 0x41, 0x54];

        let crc_calculator = CrcCalculatorAdapter::new();
        assert_eq!(0x0762E9FB, crc_calculator.calculate_crc32(input_bytes));
    }

    #[test]
    fn calculating_two_crcs_in_a_row() {
        let first_input: &[u8] = &[0x43, 0x41, 0x50, 0x4f, 0x4f];
        let second_input: &[u8] = &[0x54, 0x55, 0x54, 0x55];

        let crc_calculator = CrcCalculatorAdapter::new();

        assert_eq!(0xFC324902, crc_calculator.calculate_crc32(first_input));
        assert_eq!(0x1753C74F, crc_calculator.calculate_crc32(second_input));
    }
}