use lib_ui_framework::{Route, Screen, ScreenCreationContext};

use crate::{HomeScreen, State};

pub struct HomeRoute;

impl Route<State> for HomeRoute {
    fn create_screen(&self, ctx: ScreenCreationContext<State>) -> Box<dyn Screen> {
        Box::new(HomeScreen::new(ctx))
    }
}