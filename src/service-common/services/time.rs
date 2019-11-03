pub trait TimeService: Send {
    fn current(&mut self) -> Time;
}

#[derive(Debug)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,

    pub day: u32,
    pub month: u32,
    pub year: i32,
}