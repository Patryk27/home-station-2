use crate::{Screen, ScreenCreationContext};

pub type RouteId = &'static str;

pub trait Route {
    fn create_screen(&self, ctx: ScreenCreationContext) -> Box<dyn Screen>;
}