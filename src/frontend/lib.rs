#[macro_use]
extern crate conrod_core;

use lib_frontend_core::System;

use self::pages::*;

mod pages;

pub fn start(mut frontend: System) {
    frontend.config(move |router| {
        router.add("home", HomeRoute);
    });

    frontend.start("home");
}