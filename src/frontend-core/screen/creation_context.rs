use conrod_core::Ui;

use lib_backend::Backend;

use crate::TaskDispatcher;

pub struct ScreenCreationContext<'a> {
    pub backend: Backend,
    pub tasks: TaskDispatcher,
    pub ui: &'a mut Ui,
}