use crate::ScreenSettingContext;

pub trait Screen {
    fn update(&mut self);

    fn set(&self, ctx: ScreenSettingContext);
}