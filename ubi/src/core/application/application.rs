use crate::core::logger::init;
use crate::event::event::Event;
use crate::event::event::EventDispatcher;
use crate::ubiinfo;
use crate::window::wind_sdl::SdlWindow;
use crate::window::window_trait::{UBIWindow, WindowData};

pub struct Application<W: UBIWindow> {
    window: W,
    running: bool,
}

// Specific SDL2 window
impl Application<SdlWindow> {
    pub fn with_sdl2(window_data: WindowData) -> Self {
        init();
        let window = SdlWindow::create(window_data).unwrap();
        Self {
            window,
            running: false,
        }
    }
}

impl<W: UBIWindow> Application<W> {
    pub fn new(window: W) -> Self {
        init();
        Self {
            window,
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        let mut dispatcher = EventDispatcher::new();

        while self.running {
            let events: Vec<Event> = self.window.poll_events();

            for mut event in events {
                match &mut event {
                    Event::WindowClose(_) => {
                        dispatcher.dispatch(&mut event, |e| {
                            ubiinfo!("{}", e);
                            self.running = false;
                            true
                        });
                    }
                    Event::WindowResize(_) => {
                        dispatcher.dispatch(&mut event, |e| {
                            ubiinfo!("{}", e);
                            if let Event::WindowResize(resize_data) = e {
                                self.window.resize(resize_data.get_width(), resize_data.get_height());
                            }
                            true
                        });
                    }
                    Event::KeyPressed(_) => {
                        dispatcher.dispatch(&mut event, |e| {
                            ubiinfo!("{}", e);
                            true
                        });
                    }
                    _ => {}
                }
            }

            self.window.clear();
            self.window.swap_buffers();
        }
    }
}