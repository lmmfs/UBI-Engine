use crate::core::logger::init;
use crate::event::event::{Event, EventDispatcher};
use crate::layer::{Layer, LayerStack};
use crate::ubiinfo;
use crate::window::wind_sdl::SdlWindow;
use crate::window::window_trait::{UBIWindow, WindowData};

pub struct Application<W: UBIWindow> {
    window: W,
    running: bool,
    layer_stack: LayerStack,
}

// Specific SDL2 window
impl Application<SdlWindow> {
    pub fn with_sdl2(window_data: WindowData) -> Self {
        init();
        let window = SdlWindow::create(window_data).unwrap();
        Self {
            window,
            running: false,
            layer_stack: LayerStack::new(),
        }
    }
}

impl<W: UBIWindow> Application<W> {
    pub fn new(window: W) -> Self {
        init();
        Self {
            window,
            running: false,
            layer_stack: LayerStack::new(),
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            // Forward update layer stack
            for layer in self.layer_stack.iter_mut() {
                layer.on_update();
            }

            let events: Vec<Event> = self.window.poll_events();
            for mut event in events {
                self.on_event(&mut event);
            }

            self.window.clear();
            self.window.swap_buffers();
        }
    }

    pub fn push_layer(&mut self, mut layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, mut layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layer_stack.push_overlay(layer);
    }

    pub fn on_event(&mut self, event: &mut Event) {
        // Backward layer event handling
        for layer in self.layer_stack.iter_mut().rev() {
            layer.on_event(event);
            if event.handled() {
                break; // Stop propagating the event if a layer handles it
            }
        }

        if !event.handled() {
            let mut dispatcher = EventDispatcher::new(); // Create a local dispatcher if needed
            match event {
                Event::WindowClose(_) => {
                    dispatcher.dispatch(event, |e| {
                        ubiinfo!("{}", e);
                        self.running = false;
                        true
                    });
                }
                Event::WindowResize(_) => {
                    dispatcher.dispatch(event, |e| {
                        ubiinfo!("{}", e);
                        if let Event::WindowResize(resize_data) = e {
                            self.window
                                .resize(resize_data.get_width(), resize_data.get_height());
                        }
                        true
                    });
                }
                _ => {}
            }
        }
    }
}
