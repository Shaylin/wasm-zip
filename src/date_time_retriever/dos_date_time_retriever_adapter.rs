use crate::date_time_retriever::{DosDateTimeRetriever, SystemTimeRetriever};
use crate::date_time_retriever::chrono_system_time_retriever::ChronoSystemTimeRetriever;

pub struct DosDateTimeRetrieverAdapter {
    pub(crate) date_time_retriever: Box<dyn SystemTimeRetriever>,
}

impl DosDateTimeRetriever for DosDateTimeRetrieverAdapter {
    fn get_current_dos_time(&self) -> u16 {
        let hours = self.date_time_retriever.get_hours();
        let minutes = self.date_time_retriever.get_minutes();
        let seconds = self.date_time_retriever.get_seconds();

        let mut dos_time: u16 = 0;

        dos_time |= seconds / 2;
        dos_time |= minutes << 5;
        dos_time |= hours << 11;

        dos_time
    }

    fn get_current_dos_date(&self) -> u16 {
        let day = self.date_time_retriever.get_day();
        let month = self.date_time_retriever.get_month();
        let years_since_1980 = self.date_time_retriever.get_year() - 1980;

        let mut dos_date: u16 = 0;

        dos_date |= day;
        dos_date |= month << 5;
        dos_date |= years_since_1980 << 9;

        dos_date
    }
}

#[cfg(test)]
mod tests {
    use crate::date_time_retriever::SystemTimeRetriever;
    use super::*;

    struct FakeMidnightTimeRetriever {}

    impl SystemTimeRetriever for FakeMidnightTimeRetriever {
        fn get_hours(&self) -> u16 {
            0
        }

        fn get_minutes(&self) -> u16 {
            0
        }

        fn get_seconds(&self) -> u16 {
            0
        }

        fn get_day(&self) -> u16 {
            0
        }

        fn get_month(&self) -> u16 {
            0
        }

        fn get_year(&self) -> u16 {
            1980
        }
    }

    struct FakeMorningTimeRetriever {}

    impl SystemTimeRetriever for FakeMorningTimeRetriever {
        fn get_hours(&self) -> u16 {
            6
        }

        fn get_minutes(&self) -> u16 {
            11
        }

        fn get_seconds(&self) -> u16 {
            33
        }

        fn get_day(&self) -> u16 {
            10
        }

        fn get_month(&self) -> u16 {
            7
        }

        fn get_year(&self) -> u16 {
            1995
        }
    }

    struct FakeNightTimeRetriever {}

    impl SystemTimeRetriever for FakeNightTimeRetriever {
        fn get_hours(&self) -> u16 {
            22
        }

        fn get_minutes(&self) -> u16 {
            57
        }

        fn get_seconds(&self) -> u16 {
            48
        }

        fn get_day(&self) -> u16 {
            28
        }

        fn get_month(&self) -> u16 {
            1
        }

        fn get_year(&self) -> u16 {
            2095
        }
    }

    #[test]
    fn current_dos_time_at_midnight() {
        let fake_system_time_retriever = Box::new(FakeMidnightTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0, dos_time);
    }

    #[test]
    fn current_dos_time_at_morning() {
        let fake_system_time_retriever = Box::new(FakeMorningTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0b0011000101110000, dos_time);
    }

    #[test]
    fn current_dos_time_at_night() {
        let fake_system_time_retriever = Box::new(FakeNightTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_time = dos_date_time_retriever.get_current_dos_time();

        assert_eq!(0b1011011100111000, dos_time);
    }

    #[test]
    fn current_dos_date_at_midnight() {
        let fake_system_time_retriever = Box::new(FakeMidnightTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0, dos_date);
    }

    #[test]
    fn current_dos_date_at_morning() {
        let fake_system_time_retriever = Box::new(FakeMorningTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0b0001111011101010, dos_date);
    }

    #[test]
    fn current_dos_date_at_night() {
        let fake_system_time_retriever = Box::new(FakeNightTimeRetriever {});

        let dos_date_time_retriever = DosDateTimeRetrieverAdapter {
            date_time_retriever: fake_system_time_retriever
        };

        let dos_date = dos_date_time_retriever.get_current_dos_date();

        assert_eq!(0b1110011000111100, dos_date);
    }
}