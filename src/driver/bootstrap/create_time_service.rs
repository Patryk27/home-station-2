use lib_service_common::TimeService;
use lib_service_providers::{chrono, dummy};

use crate::config::{app::TimeServiceName, Config};

pub fn create_time_service(config: &Config) -> Box<dyn TimeService> {
    match config.app.time_service {
        TimeServiceName::Chrono => {
            Box::new(chrono::Service::new())
        }

        TimeServiceName::Dummy => {
            Box::new(dummy::Provider::new())
        }
    }
}