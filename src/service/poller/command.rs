use std::sync::mpsc::{Receiver, Sender};

pub type ServicePollerCommandTx = Sender<ServicePollerCommand>;
pub type ServicePollerCommandRx = Receiver<ServicePollerCommand>;

#[derive(Debug, Eq, PartialEq)]
pub enum ServicePollerCommand {
    Terminate,
}