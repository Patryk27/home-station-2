use std::sync::mpsc::{Receiver, SyncSender};

pub type PollCommandTx = SyncSender<PollCommand>;
pub type PollCommandRx = Receiver<PollCommand>;

#[derive(Debug, Eq, PartialEq)]
pub enum PollCommand {
    Stop,
}