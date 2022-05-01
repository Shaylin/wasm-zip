pub mod dos_date_time_retriever_adapter;
pub mod chrono_system_time_retriever;
pub mod fake_time_retriever;

pub trait SystemTimeRetriever {
    fn get_hours(&self) -> u16;
    fn get_minutes(&self) -> u16;
    fn get_seconds(&self) -> u16;
    fn get_day(&self) -> u16;
    fn get_month(&self) -> u16;
    fn get_year(&self) -> u16;
}

pub trait DosDateTimeRetriever {
    fn get_current_dos_time(&self) -> u16;
    fn get_current_dos_date(&self) -> u16;
}