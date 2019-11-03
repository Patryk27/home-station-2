use std::sync::mpsc::{channel, Sender};

use lib_service_common::Time;

use crate::TimeRequest;

#[derive(Clone)]
pub struct TimeClient {
    tx: Sender<TimeRequest>,
}

impl TimeClient {
    pub fn new(tx: Sender<TimeRequest>) -> Self {
        Self { tx }
    }

    pub fn get_time(&self) -> Time {
        let (tx, rx) = channel();

        self.tx
            .send(TimeRequest::GetTime { tx })
            .unwrap();

        rx.recv().unwrap()
    }
}