use glium::Surface;

use crate::*;

pub struct System<State> {
    config: Config,
    state: Option<State>,
    router: Router<State>,
}

impl<State> System<State> {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            router: Router::new(),
            state: None,
        }
    }

    pub fn setup_state(&mut self, callback: impl FnOnce(&mut Option<State>)) {
        callback(&mut self.state);
    }

    pub fn setup_router(&mut self, callback: impl FnOnce(&mut Router<State>)) {
        callback(&mut self.router);
    }

    pub fn start(self, main: RouteId) {
        let System { config, state, router } = self;

        let mut state = state.expect("system has not been properly initialized - did you forget to invoke `setup_state`?");

        let assets = locate_assets();
        let glium = init_glium(&config);
        let conrod = init_conrod(&config, &assets, &glium);

        // Extract bootstrap contexts
        let GliumBootstrapContext { events_loop, display } = glium;
        let ConrodBootstrapContext { mut ui, mut renderer, image_map } = conrod;

        // Initialize scheduler
        let (task_dispatcher, mut scheduler) = Scheduler::new(
            events_loop, &display,
        );

        // Initialize texture controller
        let mut texture_ctrl = TextureController::new(&display, image_map);

        texture_ctrl.initialize(&assets);

        // Initialize navigation controller
        let mut navigation_ctrl = NavigationController::new(&router);

        // Navigate to the home page
        navigation_ctrl.navigate_to(ScreenCreationContext {
            state: &mut state,
            tasks: task_dispatcher,
            ui: &mut ui,
        }, main);

        debug!("Entering event loop");

        // Spin the event loop
        while scheduler.run() {
            // Refresh interface
            let ui = &mut ui.set_widgets();

            if let Some(screen) = navigation_ctrl.current_mut() {
                screen.update();

                screen.set(ScreenSettingContext {
                    ui,
                    texture_ctrl: &texture_ctrl,
                });
            }

            // Render interface
            if let Some(primitives) = ui.draw_if_changed() {
                let image_map = texture_ctrl.image_map();

                renderer.fill(&display.0, primitives, &image_map);

                let mut frame = display.0.draw();

                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                renderer
                    .draw(&display.0, &mut frame, &image_map)
                    .expect("failed to draw UI: ");

                frame
                    .finish()
                    .expect("failed to finish drawing UI: ");
            }
        }
    }
}