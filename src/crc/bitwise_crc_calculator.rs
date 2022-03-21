use crate::crc::CrcCalculator;

pub struct BitwiseCrcCalculator {}

impl BitwiseCrcCalculator {
    fn give_two(&self) -> u32 {
        return 2;
    }
}

impl CrcCalculator for BitwiseCrcCalculator {
    fn calculate_crc32(&self) -> u32 {
        return self.give_two();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let crc_calculator = BitwiseCrcCalculator {};
        assert_eq!(2, crc_calculator.calculate_crc32());
    }
}