use crate::ubiinfo;
use crate::core::logger::init;
use crate::window::window_trait::{Window, WindowData};
use crate::window::windsdl::Windsdl;
use crate::event::{event::EventDispatcher, key_event::KeyPressedEvent};
use crate::event::application_event::*;

pub struct Application<W: Window> {
    window: W,
    running: bool,
}

// Specific SDL2 window 
impl Application<Windsdl> { 
    pub fn with_sdl2(window_data: WindowData) -> Self {
        init();
        let window = Windsdl::create(window_data).unwrap();
        Self {
            window,
            running: false,
        }
    }
}

impl<W: Window> Application<W> {
    pub fn new(window: W) -> Self {
        init(); 
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
                    ubiinfo!("{}", e);  
                    true // mark as handled
                });

                // Example: handle window close
                dispatcher.dispatch::<WindowCloseEvent, _>(|e| {
                    ubiinfo!("{}", e);
                    self.running = false;
                    true
                });

                dispatcher.dispatch::<WindowResizeEvent, _>(|e| {
                    ubiinfo!("{}", e);
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

