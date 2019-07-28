use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use conrod_core::Widget;
use conrod_core::widget::Canvas;

use lib_backend::{Backend, PollToken};
use lib_backend_common::{TimeStatus, WeatherStatus};
use lib_frontend_core::{Screen, ScreenCreationContext, ScreenSettingContext};

use self::widgets::*;

mod widgets;

pub struct HomeScreen {
    ids: Ids,

    time: TimeWidget,
    _time_token: PollToken,

    weather: WeatherWidget,
    _weather_token: PollToken,

    events: Receiver<HomeEvent>,
}

#[derive(Debug)]
enum HomeEvent {
    UpdateTime(TimeStatus),
    UpdateWeather(WeatherStatus),
}

impl HomeScreen {
    pub fn new(ScreenCreationContext { backend, ui, .. }: ScreenCreationContext) -> Self {
        let (event_tx, event_rx) = channel();

        // Start the time timer
        let time_token = {
            let event_tx = event_tx.clone();

            backend
                .poll()
                .interval(Duration::from_millis(500))
                .function(move |backend: &mut Backend| {
                    event_tx.send(HomeEvent::UpdateTime(
                        backend.time()
                    )).is_ok()
                })
                .start()
        };

        // Start the weather timer
        let weather_token = backend
            .poll()
            .interval(Duration::from_secs(5 * 60))
            .function(move |backend: &mut Backend| {
                event_tx.send(HomeEvent::UpdateWeather(
                    backend.weather()
                )).is_ok()
            })
            .start();

        Self {
            ids: Ids::new(ui.widget_id_generator()),

            time: TimeWidget::new(ui),
            _time_token: time_token,

            weather: WeatherWidget::new(ui),
            _weather_token: weather_token,

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
