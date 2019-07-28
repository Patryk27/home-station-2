use conrod_core::UiCell;

use crate::TextureController;

pub struct ScreenSettingContext<'app, 'tmp> {
    pub ui: &'tmp mut UiCell<'app>,
    pub texture_ctrl: &'tmp TextureController<'app>,
}