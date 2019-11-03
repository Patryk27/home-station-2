use crate::{Screen, ScreenCreationContext};

pub type RouteId = &'static str;

pub trait Route<State> {
    fn create_screen(&self, ctx: ScreenCreationContext<State>) -> Box<dyn Screen>;
}