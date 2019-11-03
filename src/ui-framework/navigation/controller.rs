use std::collections::VecDeque;
use std::ops::DerefMut;

use crate::{Router, Screen, ScreenCreationContext};

pub struct NavigationController<'sys, State> {
    router: &'sys Router<State>,
    screens: VecDeque<Box<dyn Screen + 'static>>,
}

impl<'sys, State> NavigationController<'sys, State> {
    pub fn new(router: &'sys Router<State>) -> Self {
        Self {
            router,
            screens: VecDeque::new(),
        }
    }

    pub fn navigate_to(&mut self, context: ScreenCreationContext<State>, id: &str) {
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