use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::{Duration, Instant};

use conrod_core::event::Input;
use conrod_core::input::{Button, Key};
use glium::glutin;

use crate::Display;

pub use self::{
    event::*,
    task::*,
    task_dispatcher::*,
};
use self::{
    event_loop::*,
    task_loop::*,
};

mod event;
mod event_loop;

mod task;
mod task_loop;
mod task_dispatcher;

const MAX_FPS: f32 = 30.0;
const MAX_PARK_DURATION: Duration = Duration::from_millis((1000.0 / MAX_FPS) as u64);

/// Scheduler is responsible for gathering and responding to external events and internal tasks.
pub struct Scheduler<'sys> {
    events: EventLoop<'sys>,
    tasks: TaskLoop,
    parked_at: Option<Instant>,
}

impl<'sys> Scheduler<'sys> {
    pub fn new(events_loop: glutin::EventsLoop, display: &'sys Display) -> (TaskDispatcher, Self) {
        // Create event loop
        let event_loop = EventLoop::new(events_loop, display);

        // Create task loop
        let (task_tx, task_rx) = channel();
        let task_loop = TaskLoop::new(task_rx);
        let task_dispatcher = TaskDispatcher::new(task_tx);

        (task_dispatcher, Self {
            events: event_loop,
            tasks: task_loop,
            parked_at: None,
        })
    }

    /// Processes all the pending events, tasks and optionally parks the scheduler (to throttle
    /// framerate).
    ///
    /// Returns `true` if application can continue working and `false` when it should be terminated.
    pub fn run(&mut self) -> bool {
        if !self.process_events() {
            return false;
        }

        self.process_tasks();
        self.park();

        true
    }

    /// Processes all the pending events.
    fn process_events(&mut self) -> bool {
        for event in self.events.iter() {
            match event {
                Event::Input(input) => {
                    if input == Input::Press(Button::Keyboard(Key::Escape)) {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Processes all the pending tasks.
    fn process_tasks(&mut self) {
        for task in self.tasks.iter() {
            // @todo
        }
    }

    /// Throttles the scheduler, so that we don't utilize the 100% CPU.
    fn park(&mut self) {
        if let Some(parked_at) = self.parked_at.take() {
            let park_duration = Instant::now().duration_since(parked_at);

            if park_duration < MAX_PARK_DURATION {
                sleep(MAX_PARK_DURATION - park_duration);
            }
        }

        self.parked_at = Some(
            Instant::now(),
        );
    }
}