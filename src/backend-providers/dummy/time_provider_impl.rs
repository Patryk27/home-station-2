use lib_backend_common::{TimeProvider, TimeStatus};

use crate::Provider;

impl TimeProvider for Provider {
    fn name(&self) -> &'static str {
        "dummy"
    }

    fn current(&mut self) -> TimeStatus {
        TimeStatus {
            hour: 12,
            minute: 0,
            second: 0,

            day: 1,
            month: 1,
            year: 1980,

            human_weekday: "Monday".to_string(),
            human_month: "January".to_string(),
        }
    }
}