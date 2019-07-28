use crate::{Task, TaskTx};

pub struct TaskDispatcher {
    tx: TaskTx,
}

impl TaskDispatcher {
    pub(crate) fn new(tx: TaskTx) -> Self {
        Self { tx }
    }

    pub fn enqueue(&self, task: Task) {
        let _ = self.tx.send(task);
    }
}