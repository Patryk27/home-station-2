pub trait TimeProvider: Send {
    fn name(&self) -> &'static str;
    fn current(&mut self) -> TimeStatus;
}

#[derive(Debug)]
pub struct TimeStatus {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,

    pub day: u32,
    pub month: u32,
    pub year: i32,

    pub human_weekday: String,
    pub human_month: String,
}