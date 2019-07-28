use std::collections::VecDeque;
use std::ops::DerefMut;

use crate::{Router, Screen, ScreenCreationContext};

pub struct NavigationController<'a> {
    router: &'a Router,
    screens: VecDeque<Box<dyn Screen + 'static>>,
}

impl<'a> NavigationController<'a> {
    pub fn new(router: &'a Router) -> Self {
        Self {
            router,
            screens: VecDeque::new(),
        }
    }

    pub fn navigate_to(&mut self, context: ScreenCreationContext, id: &str) {
        debug!("Navigating to: {}", id);

        let screen = self.router
            .get(id)
            .unwrap()
            .create_screen(context);

        self.screens.push_back(screen);
    }

    pub fn current_mut(&mut self) -> Option<&mut (dyn Screen + 'static)> {
        self.screens
            .back_mut()
            .map(DerefMut::deref_mut)
    }
}