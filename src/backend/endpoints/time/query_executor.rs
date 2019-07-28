use std::sync::mpsc::{channel, Receiver, Sender};

use lib_backend_common::TimeProvider;

use super::TimeQuery;

pub struct TimeQueryExecutor {
    provider: Box<dyn TimeProvider>,
    rx: Receiver<TimeQuery>,
}

impl TimeQueryExecutor {
    pub(crate) fn new(provider: Box<dyn TimeProvider>) -> (Sender<TimeQuery>, Self) {
        let (tx, rx) = channel();

        (tx, Self { provider, rx })
    }

    pub fn start(mut self) {
        for query in self.rx.iter() {
            trace!("Processing query: {:?}", query);

            match query {
                TimeQuery::GetCurrent { tx } => {
                    tx.send(self.provider.current()).unwrap();
                }
            }
        }
    }
}