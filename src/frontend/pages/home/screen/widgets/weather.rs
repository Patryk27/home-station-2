use conrod_core::{color, Colorable, Positionable, Sizeable, Ui, Widget};
use conrod_core::widget::{Canvas, Id as WidgetId, Image, Text};

use lib_backend_common::WeatherStatus;
use lib_frontend_core::ScreenSettingContext;

pub struct WeatherWidget {
    ids: Ids,
    status: Option<WeatherStatus>,
}

impl WeatherWidget {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            status: None,
        }
    }

    pub fn update(&mut self, status: WeatherStatus) {
        self.status = Some(status);
    }

    pub fn set(&self, parent: WidgetId, ctx: &mut ScreenSettingContext) {
        if let Some(status) = &self.status {
            Canvas::new()
                .parent(parent)
                .wh_of(parent)
                .flow_right(&[
                    (self.ids.icon_wrapper, Canvas::new()),
                    (self.ids.status_wrapper, Canvas::new().length_weight(1.2)),
                ])
                .set(self.ids.main, ctx.ui);

            Canvas::new()
                .parent(self.ids.status_wrapper)
                .wh_of(self.ids.status_wrapper)
                .flow_right(&[
                    (self.ids.status_left, Canvas::new().flow_down(&[
                        (self.ids.temperature_wrapper, Canvas::new().pad_bottom(10.0)),
                        (self.ids.pressure_wrapper, Canvas::new().pad_top(10.0)),
                    ])),
                    (self.ids.status_right, Canvas::new().flow_down(&[
                        (self.ids.humidity_wrapper, Canvas::new().pad_bottom(10.0)),
                        (self.ids.wind_speed_wrapper, Canvas::new().pad_top(10.0)),
                    ])),
                ])
                .set(self.ids.status, ctx.ui);

            if let Some(icon) = &status.icon {
                if let Some(icon) = ctx.texture_ctrl.get(&format!("widgets:weather:{}", icon)) {
                    Image::new(icon)
                        .w_h(300.0, 300.0)
                        .middle_of(self.ids.icon_wrapper)
                        .set(self.ids.icon, ctx.ui);
                }
            }

            if let Some(temperature) = Self::temperature(status) {
                Text::new(&temperature)
                    .mid_bottom_of(self.ids.temperature_wrapper)
                    .color(color::WHITE)
                    .font_size(50)
                    .set(self.ids.temperature, ctx.ui);
            }

            if let Some(pressure) = Self::pressure(status) {
                Text::new(&pressure)
                    .mid_top_of(self.ids.pressure_wrapper)
                    .color(color::WHITE)
                    .font_size(50)
                    .set(self.ids.pressure, ctx.ui);
            }

            if let Some(humidity) = Self::humidity(status) {
                Text::new(&humidity)
                    .mid_bottom_of(self.ids.humidity_wrapper)
                    .color(color::WHITE)
                    .font_size(50)
                    .set(self.ids.humidity, ctx.ui);
            }

            if let Some(wind_speed) = Self::wind_speed(status) {
                Text::new(&wind_speed)
                    .mid_top_of(self.ids.wind_speed_wrapper)
                    .color(color::WHITE)
                    .font_size(50)
                    .set(self.ids.wind_speed, ctx.ui);
            }
        }
    }

    fn temperature(status: &WeatherStatus) -> Option<String> {
        Some(format!("{:.01} C", status.temperature?))
    }

    fn pressure(status: &WeatherStatus) -> Option<String> {
        Some(format!("{} hPa", status.pressure?))
    }

    fn humidity(status: &WeatherStatus) -> Option<String> {
        Some(format!("{} %", status.humidity?))
    }

    fn wind_speed(status: &WeatherStatus) -> Option<String> {
        Some(format!("{:.0} km/h", status.wind_speed?))
    }
}

widget_ids! {
    struct Ids {
        main,
            icon,
            icon_wrapper,

            status,
            status_wrapper,
                status_left,
                    temperature,
                    temperature_wrapper,

                    pressure,
                    pressure_wrapper,

                status_right,
                    humidity,
                    humidity_wrapper,

                    wind_speed,
                    wind_speed_wrapper,
    }
}
