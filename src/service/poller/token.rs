use crate::{ServicePollerCommand, ServicePollerCommandTx};

pub struct ServicePollerToken {
    tx: ServicePollerCommandTx,
}

impl ServicePollerToken {
    pub(crate) fn new(tx: ServicePollerCommandTx) -> Self {
        Self { tx }
    }
}

impl Drop for ServicePollerToken {
    fn drop(&mut self) {
        let _ = self.tx.send(ServicePollerCommand::Terminate);
    }
}