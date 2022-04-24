use chrono::{Datelike, Timelike, Utc};
use crate::date_time_retriever::SystemTimeRetriever;

pub struct ChronoSystemTimeRetriever {}

impl SystemTimeRetriever for ChronoSystemTimeRetriever {
    fn get_hours(&self) -> u16 {
        Utc::now().hour() as u16
    }

    fn get_minutes(&self) -> u16 {
        Utc::now().minute() as u16
    }

    fn get_seconds(&self) -> u16 {
        Utc::now().second() as u16
    }

    fn get_day(&self) -> u16 {
        Utc::now().day() as u16
    }

    fn get_month(&self) -> u16 {
        Utc::now().month() as u16
    }

    fn get_year(&self) -> u16 {
        Utc::now().year() as u16
    }
}