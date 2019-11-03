use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};

use crate::{Config, Display};

pub struct GliumBootstrapContext {
    pub events_loop: EventsLoop,
    pub display: Display,
}

pub fn init_glium(config: &Config) -> GliumBootstrapContext {
    debug!("Initializing Glium:");

    // Initialize window
    debug!("-> window");

    let window = WindowBuilder::new()
        .with_title("HomeStation")
        .with_dimensions((config.width, config.height).into());

    // Initialize context
    debug!("-> context");

    let context = ContextBuilder::new();

    // Initialize events loop
    debug!("-> events loop");

    let events_loop = EventsLoop::new();

    // Initialize display
    debug!("-> display");

    let display = Display(
        glium::Display::new(window, context, &events_loop).unwrap()
    );

    debug!("... ready");

    GliumBootstrapContext { events_loop, display }
}