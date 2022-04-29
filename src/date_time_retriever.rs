pub mod dos_date_time_retriever;
pub mod chrono_system_time_retriever;

pub trait SystemTimeRetriever {
    fn get_hours(&self) -> u16;
    fn get_minutes(&self) -> u16;
    fn get_seconds(&self) -> u16;
    fn get_day(&self) -> u16;
    fn get_month(&self) -> u16;
    fn get_year(&self) -> u16;
}

pub struct DosDateTimeRetriever {
    date_time_retriever: Box<dyn SystemTimeRetriever>
}