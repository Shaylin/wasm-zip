use crate::date_time_retriever::SystemTimeRetriever;

pub struct FakeTimeRetriever {}


impl SystemTimeRetriever for FakeTimeRetriever {
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
        0
    }
}