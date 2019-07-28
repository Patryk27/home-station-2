use std::sync::mpsc::sync_channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::{Backend, PollCommand, PollToken};

pub struct PollBuilder<F> {
    backend: Backend,
    interval: Option<Duration>,
    function: Option<F>,
}

impl<F> PollBuilder<F> {
    pub(crate) fn new(backend: Backend) -> Self {
        Self {
            backend,
            interval: None,
            function: None,
        }
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn function(mut self, function: F) -> Self {
        self.function = Some(function);
        self
    }
}

impl<F> PollBuilder<F> where F: FnMut(&mut Backend) -> bool + Send + 'static {
    pub fn start(self) -> PollToken {
        let mut backend = self.backend;
        let interval = self.interval.unwrap();
        let mut function = self.function.unwrap();

        let (command_tx, command_rx) = sync_channel(1);

        // @todo supervisor
        spawn(move || {
            loop {
                if command_rx.try_recv() == Ok(PollCommand::Stop) {
                    return;
                }

                if function(&mut backend) {
                    sleep(interval);
                } else {
                    break;
                }
            }
        });

        PollToken::new(command_tx)
    }
}