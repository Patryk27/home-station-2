use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use lib_service_common::TimeService;

use crate::TimeRequest;

pub struct TimeServer {
    service: Box<dyn TimeService>,
    rx: Receiver<TimeRequest>,
}

impl TimeServer {
    pub fn new(service: Box<dyn TimeService>, rx: Receiver<TimeRequest>) -> Self {
        Self { service, rx }
    }

    pub fn spawn(service: Box<dyn TimeService>) -> Sender<TimeRequest> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            Self::new(service, rx).start()
        });

        tx
    }

    pub fn start(mut self) {
        for request in self.rx.iter() {
            trace!("Processing request: {:?}", request);

            match request {
                TimeRequest::GetTime { tx } => {
                    tx.send(self.service.current()).unwrap();
                }
            }
        }
    }
}

impl Drop for TimeServer {
    fn drop(&mut self) {
        trace!("Terminating");
    }
}