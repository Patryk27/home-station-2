use crate::{PollCommand, PollCommandTx};

pub struct PollToken {
    tx: PollCommandTx,
}

impl PollToken {
    pub(crate) fn new(tx: PollCommandTx) -> Self {
        Self { tx }
    }
}

impl Drop for PollToken {
    fn drop(&mut self) {
        let _ = self.tx.send(PollCommand::Stop);
    }
}