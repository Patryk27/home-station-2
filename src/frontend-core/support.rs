use glium;

pub struct Display(pub glium::Display);

impl conrod_winit::WinitWindow for Display {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

conrod_winit::conversion_fns!();
