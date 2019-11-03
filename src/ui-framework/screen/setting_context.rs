use conrod_core::UiCell;

use crate::TextureController;

pub struct ScreenSettingContext<'sys, 'borrow> {
    pub ui: &'borrow mut UiCell<'sys>,
    pub texture_ctrl: &'borrow TextureController<'sys>,
}