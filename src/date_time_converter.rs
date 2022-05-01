pub mod dos_date_time_calculator_adapter;

pub struct SystemTime {
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub day: u16,
    pub month: u16,
    pub year: u16,
}

pub trait DosDateTimeCalculator {
    fn get_current_dos_time(&self) -> u16;
    fn get_current_dos_date(&self) -> u16;
}