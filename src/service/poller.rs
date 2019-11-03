use std::sync::mpsc::{channel, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub use self::{
    command::*,
    token::*,
};

mod command;
mod token;

pub struct ServicePoller<Context> {
    context: Context,
    interval: Option<Duration>,
}

impl<Context: Clone> ServicePoller<Context> {
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
            interval: None,
        }
    }
}

impl<Context> ServicePoller<Context> {
    pub fn each(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }
}

impl<Context: Send + 'static> ServicePoller<Context> {
    pub fn run(
        self,
        mut handler: impl FnMut(&mut Context) + Send + 'static,
    ) -> ServicePollerToken {
        let mut context = self.context;
        let interval = self.interval.unwrap();

        let (tx, rx) = channel();

        spawn(move || {
            if rx.try_recv() == Ok(ServicePollerCommand::Terminate) {
                return;
            }

            handler(&mut context);

            sleep(interval);
        });

        ServicePollerToken::new(tx)
    }

    pub fn send<Event: Send + 'static>(
        self,
        tx: &Sender<Event>,
        mut handler: impl (FnMut(&mut Context) -> Event) + Send + 'static,
    ) -> ServicePollerToken {
        let tx = tx.clone();

        self.run(move |context| {
            tx.send(handler(context)).unwrap();
        })
    }
}