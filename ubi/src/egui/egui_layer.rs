use crate::{event, ubiinfo};
use crate::event::event::EventDispatcher;
use crate::layer::Layer;
use crate::prelude::Event as UbiEvent;
use egui::{Color32, Context as EguiContext, FullOutput, RawInput, TextureId};
use egui_sdl2_gl as egui_backend;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl::{DpiScaling, EguiStateHandler, ShaderVersion};
use sdl2::event::Event;
use sdl2::video::Window;
use std::rc::Rc;
use std::time::Instant;

pub struct EguiLayer {
    pub ctx: EguiContext,
    pub raw_input: RawInput,
    pub start_time: std::time::Instant,
    pub painter: Painter,
    pub egui_state: EguiStateHandler,
    pub window: Rc<Window>,
}

impl EguiLayer {
    pub fn new(window: Rc<Window>) -> Self {
        let (painter, egui_state) =
            egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);

        Self {
            ctx: EguiContext::default(),
            raw_input: RawInput::default(),
            start_time: Instant::now(),
            painter,
            egui_state: egui_state,
            window,
        }
    }
}

impl Layer for EguiLayer {
    fn on_attach(&mut self) {
        self.start_time = std::time::Instant::now();
    }

    fn on_detach(&mut self) {
        todo!()
    }

    fn on_update(&mut self, events: &mut Vec<UbiEvent>) {
        // Egui frame preparation (should be done before any UI drawing)
        self.egui_state.input.time = Some(self.start_time.elapsed().as_secs_f64());
        self.ctx.begin_frame(self.egui_state.input.take());

        // Your Egui UI code goes here
        egui::Window::new("Egui with SDL2 and GL").show(&self.ctx, |ui| {
            ui.label("A simple Label.");
            if ui.button("Click me!").clicked() {
                println!("Button clicked!");
            }
            ui.checkbox(&mut true, "A checkbox"); // Example interactive element
        });

        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output, // You might need to handle this if you use egui viewports
        } = self.ctx.end_frame();

        // Process egui's output (e.g., set mouse cursor, open URLs)
        self.egui_state
            .process_output(&self.window, &platform_output);

        // Convert egui shapes into paint jobs and paint them
        let paint_jobs = self.ctx.tessellate(shapes, pixels_per_point);
        self.painter.paint_jobs(None, textures_delta, paint_jobs);
    }

    fn on_event(&mut self, event: &mut crate::prelude::Event) {
        let mut dispatcher = EventDispatcher::new();
        match event {
            crate::prelude::Event::MouseButtonPressed(data) => {
                dispatcher.dispatch(event, |e| {
                    println!("{}", e);
                    ubiinfo!("{}", e);
                    true
                });
            }

            _ => {}
        }
    }
}
