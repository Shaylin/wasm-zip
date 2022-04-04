use crate::crc_calculator::CrcCalculator;

pub struct CrcCalculatorAdapter {}

impl CrcCalculatorAdapter {
    fn give_two(&self) -> u32 {
        return 2;
    }
}

impl CrcCalculator for CrcCalculatorAdapter {
    fn calculate_crc32(&self, data: &[u8]) -> u32 {
        return self.give_two();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let crc_calculator = CrcCalculatorAdapter {};
        assert_eq!(2, crc_calculator.calculate_crc32());
    }
}