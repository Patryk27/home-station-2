use std::sync::mpsc::Sender;

use lib_service_common::Time;

#[derive(Debug)]
pub enum TimeRequest {
    GetTime {
        tx: Sender<Time>,
    },
}