use crate::layer::Layer;
use egui::{Color32, Context as EguiContext, FullOutput, RawInput, TextureId};
use egui_sdl2_gl as egui_backend;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl::{DpiScaling, EguiStateHandler, ShaderVersion};
use sdl2::video::Window;
use std::rc::Rc;
use std::time::Instant;
use crate::prelude::Event as UbiEvent;

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
        let start_time = Instant::now();
        self.egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        self.ctx.begin_frame(self.egui_state.input.take());
        let mut quit = false;

        egui::Window::new("Egui with SDL2 and GL").show(&self.ctx, |ui| {
            ui.label("A simple Label.");
        });

        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = self.ctx.end_frame();

        self.egui_state
            .process_output(&self.window, &platform_output);

        let paint_jobs = self.ctx.tessellate(shapes, pixels_per_point);

        self.painter.paint_jobs(None, textures_delta, paint_jobs);
    }

    fn on_event(&mut self, _event: &mut crate::prelude::Event) {}
}
