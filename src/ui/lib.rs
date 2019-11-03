#[macro_use]
extern crate conrod_core;

use lib_ui_framework::System;
pub use state::State;

use self::pages::*;

mod pages;
mod state;

pub fn start(mut system: System<State>, state: State) {
    system.setup_state(move |sys_state| {
        *sys_state = Some(state);
    });

    system.setup_router(move |router| {
        router.add("home", HomeRoute);
    });

    system.start("home");
}