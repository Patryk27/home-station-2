use std::sync::mpsc::{Receiver, Sender};

pub type TaskTx = Sender<Task>;
pub type TaskRx = Receiver<Task>;

#[derive(Debug)]
pub enum Task {
    // Navigation-related commands
    NavigateTo {
        id: String,
    },
}