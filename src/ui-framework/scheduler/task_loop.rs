use crate::{Task, TaskRx};

pub struct TaskLoop {
    rx: TaskRx,
}

impl TaskLoop {
    pub fn new(rx: TaskRx) -> Self {
        Self { rx }
    }

    pub fn iter(&mut self) -> impl Iterator<Item=Task> + '_ {
        self.rx.try_iter()
    }
}