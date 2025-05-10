use crate::core::custom_error::UbiError;
use crate::core::logger::init;
use crate::event::event::{Event, EventDispatcher};
use crate::graphics::render::Renderer;
use crate::layer::{Layer, LayerStack};
use crate::ubiinfo;
use crate::window::wind_sdl::SdlWindow;
use crate::window::window_trait::{UBIWindow, WindowData};

pub struct Application<W: UBIWindow> {
    window: W,
    running: bool,
    layer_stack: LayerStack,
    renderer: Renderer,
}

// Specific SDL2 window
impl Application<SdlWindow> {
    pub fn with_sdl2(window_data: WindowData) -> Self {
        init();
        let window = SdlWindow::create(window_data).unwrap();
        let renderer = Renderer::new().unwrap();
        Self {
            window,
            running: false,
            layer_stack: LayerStack::new(),
            renderer: renderer,
        }
    }
}

impl<W: UBIWindow> Application<W> {
    pub fn new(window: W) -> Self {
        init();
        let renderer = Renderer::new().unwrap();
        Self {
            window,
            running: false,
            layer_stack: LayerStack::new(),
            renderer: renderer,
        }
    }

    pub fn run(&mut self) -> Result<(), UbiError> {
        self.running = true;
        let mut events: Vec<crate::event::event::Event> = Vec::new();
        while self.running {
            // clear events and screen
            events.clear();
            self.window.clear();

            // Forward update layer stack
            for layer in self.layer_stack.iter_mut() {
                layer.on_update(&mut events);
            }

            match self.window.poll_events(&mut events) {
                Ok(_) => {
                    for event in &mut events {
                        self.on_event(event);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
            
            self.window.swap_buffers();
            
        }
        Ok(())
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
                // Stop propagating the event if a layer handles it
                break;
            }
        }

        if !event.handled() {
            let mut dispatcher = EventDispatcher::new();
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
