#[macro_use]
extern crate log;

use chrono::{Datelike, Local, Timelike};

use lib_service_common::{Time, TimeService};

pub struct Service;

impl Service {
    pub fn new() -> Self {
        trace!("Initializing Chrono");

        Self
    }
}

impl TimeService for Service {
    fn current(&mut self) -> Time {
        let now = Local::now();

        Time {
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),

            day: now.day(),
            month: now.month(),
            year: now.year(),
        }
    }
}