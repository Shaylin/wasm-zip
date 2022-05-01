use crate::date_time_converter::{DosDateTimeCalculator, SystemTime};

pub struct DosDateTimeCalculatorAdapter {
    pub(crate) date_time: SystemTime,
}

impl DosDateTimeCalculator for DosDateTimeCalculatorAdapter {
    fn get_current_dos_time(&self) -> u16 {
        let hours = self.date_time.hours;
        let minutes = self.date_time.minutes;
        let seconds = self.date_time.seconds;

        let mut dos_time: u16 = 0;

        dos_time |= seconds / 2;
        dos_time |= minutes << 5;
        dos_time |= hours << 11;

        dos_time
    }

    fn get_current_dos_date(&self) -> u16 {
        let day = self.date_time.day;
        let month = self.date_time.month;
        let years_since_1980 = self.date_time.year - 1980;

        let mut dos_date: u16 = 0;

        dos_date |= day;
        dos_date |= month << 5;
        dos_date |= years_since_1980 << 9;

        dos_date
    }
}

#[cfg(test)]
mod tests {
    use crate::date_time_converter::SystemTime;

    use super::*;

    #[test]
    fn current_dos_time_at_midnight() {
        let fake_system_time = SystemTime {
            seconds: 0,
            minutes: 0,
            hours: 0,
            day: 0,
            month: 0,
            year: 1980,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0, dos_time);
    }

    #[test]
    fn current_dos_time_at_morning() {
        let fake_system_time = SystemTime {
            seconds: 33,
            minutes: 11,
            hours: 6,
            day: 10,
            month: 7,
            year: 1995,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0b0011000101110000, dos_time);
    }

    #[test]
    fn current_dos_time_at_night() {
        let fake_system_time = SystemTime {
            seconds: 48,
            minutes: 57,
            hours: 22,
            day: 28,
            month: 1,
            year: 2095,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0b1011011100111000, dos_time);
    }

    #[test]
    fn current_dos_date_at_midnight() {
        let fake_system_time = SystemTime {
            seconds: 0,
            minutes: 0,
            hours: 0,
            day: 0,
            month: 0,
            year: 1980,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0, dos_date);
    }

    #[test]
    fn current_dos_date_at_morning() {
        let fake_system_time = SystemTime {
            seconds: 33,
            minutes: 11,
            hours: 6,
            day: 10,
            month: 7,
            year: 1995,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0b0001111011101010, dos_date);
    }

    #[test]
    fn current_dos_date_at_night() {
        let fake_system_time = SystemTime {
            seconds: 48,
            minutes: 57,
            hours: 22,
            day: 28,
            month: 1,
            year: 2095,
        };

        let dos_date_time_retriever = DosDateTimeCalculatorAdapter {
            date_time: fake_system_time
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0b1110011000111100, dos_date);
    }
}