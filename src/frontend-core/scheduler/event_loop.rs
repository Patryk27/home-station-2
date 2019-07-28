use glium::glutin;

use crate::{Display, Event};

pub struct EventLoop<'a> {
    events_loop: glutin::EventsLoop,
    display: &'a Display,
    events: Vec<Event>,
}

impl<'a> EventLoop<'a> {
    pub fn new(events_loop: glutin::EventsLoop, display: &'a Display) -> Self {
        Self {
            events_loop,
            display,
            events: Vec::new(),
        }
    }

    pub fn iter(&mut self) -> impl Iterator<Item=Event> + '_ {
        let display = &self.display;
        let events = &mut self.events;

        self.events_loop.poll_events(|event| {
            if let Some(event) = transform(event, display) {
                events.push(event);
            }
        });

        self.events.drain(..)
    }
}

fn transform(event: glutin::Event, display: &Display) -> Option<Event> {
    if let Some(event) = crate::support::convert_event(event, display) {
        return Some(Event::Input(event));
    }

    None
}