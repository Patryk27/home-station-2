use conrod_core::{color, Colorable, Positionable, Sizeable, Ui, Widget};
use conrod_core::position::Relative::Scalar;
use conrod_core::widget::{Canvas, Id as WidgetId, Text};

use lib_service_common::Time;
use lib_ui_framework::ScreenSettingContext;

pub struct TimeWidget {
    ids: Ids,
    status: Option<Time>,
}

impl TimeWidget {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            status: None,
        }
    }

    pub fn update(&mut self, status: Time) {
        self.status = Some(status);
    }

    pub fn set(&self, parent: WidgetId, ctx: &mut ScreenSettingContext) {
        if let Some(status) = &self.status {
            Canvas::new()
                .parent(parent)
                .wh_of(parent)
                .flow_right(&[
                    (self.ids.main_left, Canvas::new().flow_down(&[
                        (self.ids.time_wrapper, Canvas::new()),
                    ])),
                    (self.ids.main_right, Canvas::new().flow_down(&[
                        (self.ids.weekday_wrapper, Canvas::new().pad_bottom(15.0)),
                        (self.ids.date_wrapper, Canvas::new().pad_top(0.0)),
                    ]).length_weight(1.15)),
                ])
                .set(self.ids.main, ctx.ui);

            Text::new(&Self::time(status))
                .middle_of(self.ids.time_wrapper)
                .y_position_relative(Scalar(15.0))
                .color(color::WHITE)
                .font_size(140)
                .set(self.ids.time, ctx.ui);

            // @todo
//            Text::new(&status.human_weekday)
//                .mid_bottom_of(self.ids.weekday_wrapper)
//                .color(color::WHITE)
//                .font_size(50)
//                .set(self.ids.weekday, ctx.ui);

            Text::new(&Self::date(status))
                .mid_top_of(self.ids.date_wrapper)
                .color(color::WHITE)
                .font_size(50)
                .set(self.ids.date, ctx.ui);
        }
    }

    fn time(status: &Time) -> String {
        let separator = if status.second % 2 == 0 { ' ' } else { ':' };

        format!("{:02}{}{:02}", status.hour, separator, status.minute)
    }

    fn date(status: &Time) -> String {
        format!("{} {}", status.day, "----") // @todo
    }
}

widget_ids! {
    struct Ids {
        main,
            main_left,
                time,
                time_wrapper,

            main_right,
                weekday,
                weekday_wrapper,

                date,
                date_wrapper,
    }
}
