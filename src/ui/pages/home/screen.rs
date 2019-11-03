use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use conrod_core::Widget;
use conrod_core::widget::Canvas;

use lib_service::{ServicePoller, ServicePollerToken};
use lib_service_common::{Time, Weather};
use lib_ui_framework::{Screen, ScreenCreationContext, ScreenSettingContext};

use crate::State;

use self::widgets::*;

mod widgets;

pub struct HomeScreen {
    ids: Ids,

    time: TimeWidget,
    _time_token: ServicePollerToken,

    weather: WeatherWidget,
    _weather_token: ServicePollerToken,

    events: Receiver<HomeEvent>,
}

#[derive(Debug)]
enum HomeEvent {
    UpdateTime(Time),
    UpdateWeather(Weather),
}

impl HomeScreen {
    pub fn new(ScreenCreationContext { state, ui, .. }: ScreenCreationContext<State>) -> Self {
        let (event_tx, event_rx) = channel();

        // @todo move all the ServicePoller logic to the Scheduler;
        //       thanks to this we'll be able to skip all the token fiddling and do something like
        //       scoped polling

        let _time_token = ServicePoller::new(&state.time)
            .each(Duration::from_millis(500))
            .send(&event_tx, move |time| {
                HomeEvent::UpdateTime(time.get_time())
            });

        let _weather_token = ServicePoller::new(&state.weather)
            .each(Duration::from_millis(500))
            .send(&event_tx, move |weather| {
                HomeEvent::UpdateWeather(weather.get_weather())
            });

        Self {
            ids: Ids::new(ui.widget_id_generator()),

            time: TimeWidget::new(ui),
            _time_token,

            weather: WeatherWidget::new(ui),
            _weather_token,

            events: event_rx,
        }
    }
}

impl Screen for HomeScreen {
    fn update(&mut self) {
        for event in self.events.try_iter() {
            match event {
                HomeEvent::UpdateTime(time) => {
                    self.time.update(time);
                }

                HomeEvent::UpdateWeather(weather) => {
                    self.weather.update(weather);
                }
            }
        }
    }

    fn set(&self, mut ctx: ScreenSettingContext) {
        Canvas::new()
            .flow_down(&[
                (self.ids.time, Canvas::new()),
                (self.ids.weather, Canvas::new().length_weight(2.0)),
            ])
            .set(self.ids.main, ctx.ui);

        self.time.set(self.ids.time, &mut ctx);
        self.weather.set(self.ids.weather, &mut ctx);
    }
}

widget_ids! {
    struct Ids {
        main,
            time,
            weather,
    }
}
