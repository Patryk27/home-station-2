use chrono::{Datelike, Local, Timelike};

use lib_backend_common::{TimeProvider, TimeStatus};

use crate::Provider;

impl TimeProvider for Provider {
    fn name(&self) -> &'static str {
        "chrono"
    }

    fn current(&mut self) -> TimeStatus {
        let now = Local::now();

        TimeStatus {
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),

            day: now.day(),
            month: now.month(),
            year: now.year(),

            human_weekday: self.config.weekdays[now.weekday().num_days_from_monday() as usize].to_owned(),
            human_month: self.config.months[now.month0() as usize].to_owned(),
        }
    }
}