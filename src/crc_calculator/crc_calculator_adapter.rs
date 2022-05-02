use crate::crc_calculator::CrcCalculator;
use crc::{Crc};

pub struct CrcCalculatorAdapter {
    pub(crate) crc: Crc<u32>,
}

impl CrcCalculator for CrcCalculatorAdapter {
    fn calculate_crc32(&self, data: &[u8]) -> u32 {
        let mut digest = self.crc.digest();

        digest.update(data);

        digest.finalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::CRC_32_ISO_HDLC;
    use super::*;

    #[test]
    fn single_byte() {
        let input_byte: &[u8] = &[0x42];

        let crc_calculator = CrcCalculatorAdapter {
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
        };

        assert_eq!(0x4AD0CF31, crc_calculator.calculate_crc32(input_byte));
    }

    #[test]
    fn multiple_bytes() {
        let input_bytes: &[u8] = &[0x42, 0x55, 0x47, 0x43, 0x41, 0x54];

        let crc_calculator = CrcCalculatorAdapter {
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
        };

        assert_eq!(0xF2DAF2CB, crc_calculator.calculate_crc32(input_bytes));
    }

    #[test]
    fn calculating_two_crcs_in_a_row() {
        let first_input: &[u8] = &[0x43, 0x41, 0x50, 0x4f, 0x4f];
        let second_input: &[u8] = &[0x54, 0x55, 0x54, 0x55];

        let crc_calculator = CrcCalculatorAdapter {
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC)
        };

        assert_eq!(0x7E7CBA7, crc_calculator.calculate_crc32(first_input));
        assert_eq!(0xCA8A9699, crc_calculator.calculate_crc32(second_input));
    }
}