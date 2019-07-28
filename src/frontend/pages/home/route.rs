use lib_frontend_core::{Route, Screen, ScreenCreationContext};

use crate::HomeScreen;

pub struct HomeRoute;

impl Route for HomeRoute {
    fn create_screen(&self, ctx: ScreenCreationContext) -> Box<dyn Screen> {
        Box::new(HomeScreen::new(ctx))
    }
}