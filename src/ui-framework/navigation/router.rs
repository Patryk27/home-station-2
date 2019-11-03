use std::collections::HashMap;
use std::ops::Deref;

use crate::{Route, RouteId};

pub struct Router<State> {
    routes: HashMap<RouteId, Box<dyn Route<State>>>,
}

impl<State> Router<State> {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn add(&mut self, id: RouteId, route: impl Route<State> + 'static) {
        self.routes.insert(id, Box::new(route));
    }

    pub fn get(&self, id: &str) -> Option<&dyn Route<State>> {
        self.routes
            .get(id)
            .map(Deref::deref)
    }
}