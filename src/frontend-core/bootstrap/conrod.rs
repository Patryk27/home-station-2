use std::path::PathBuf;

use conrod_core::{image, Ui, UiBuilder};
use conrod_glium::Renderer;
use glium::texture::Texture2d;

use crate::{Config, GliumBootstrapContext};

pub struct ConrodBootstrapContext {
    pub ui: Ui,
    pub renderer: Renderer,
    pub image_map: image::Map<Texture2d>,
}

pub fn init_conrod(
    config: &Config,
    assets: &PathBuf,
    glium: &GliumBootstrapContext,
) -> ConrodBootstrapContext {
    debug!("Initializing Conrod:");

    // Initialize core
    debug!("-> core");

    let mut ui = UiBuilder::new([config.width as f64, config.height as f64]).build();
    let renderer = Renderer::new(&glium.display.0).unwrap();
    let image_map = image::Map::<Texture2d>::new();

    // Load fonts
    debug!("-> fonts");

    let fonts = assets
        .join("app")
        .join("fonts");

    for font in &["FiraSans-Light.ttf", "FiraSans-Regular.ttf"] {
        ui.fonts
            .insert_from_file(fonts.join(font))
            .unwrap();
    }

    debug!("... ready");

    ConrodBootstrapContext { ui, renderer, image_map }
}