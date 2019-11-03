use conrod_core::Ui;

use crate::TaskDispatcher;

pub struct ScreenCreationContext<'sys, State> {
    pub state: &'sys mut State,
    pub tasks: TaskDispatcher,
    pub ui: &'sys mut Ui,
}