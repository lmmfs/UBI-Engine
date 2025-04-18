use crate::window::window_trait::Window;
use crate::event::{event::Event, event::EventDispatcher, key_event::KeyPressedEvent};
use crate::event::application_event::*;

pub struct Application<W: Window> {
    window: W,
    running: bool,
}

impl<W: Window> Application<W> {
    pub fn new(window: W) -> Self {
        Self {
            window,
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            let events = self.window.poll_events(); // returns Vec<Box<dyn Event>>

            for event in events {
                let mut dispatcher = EventDispatcher::new(event); // your custom dispatcher

                // Example: handle key pressed
                dispatcher.dispatch::<KeyPressedEvent, _>(|e| {
                    println!("Key Pressed: {}", e.get_key_code());
                    true // mark as handled
                });

                // Example: handle window close
                dispatcher.dispatch::<WindowCloseEvent, _>(|_e| {
                    self.running = false;
                    true
                });

                dispatcher.dispatch::<WindowResizeEvent, _>(|e| {
                    self.window.resize(e.get_width(), e.get_height());
                    true
                });
            }

            // your rendering, logic, update, etc
            self.window.clear();
            self.window.swap_buffers(); 
        }
    }


}

