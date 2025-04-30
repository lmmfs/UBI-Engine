#![allow(unused)]
#[allow(dead_code)]
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes};

use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};

use glutin_winit::{DisplayBuilder, GlWindow};

// ubi imports
use crate::event::application_event::*;
use crate::ubiinfo;
use crate::window::window_trait::{UBIWindow, WindowData};

pub struct WinitWindow {
    pub window: Window,
    context: glutin::context::PossiblyCurrentContext,
    surface: glutin::surface::Surface<WindowSurface>,
    event_loop: EventLoop<()>,
}

impl UBIWindow for WinitWindow {
    fn create(window_data: crate::prelude::WindowData) -> Result<Self, String>
    where
        Self: Sized,
    {
        let event_loop = EventLoop::new().unwrap();

        // Create window
        let window_builder = WindowBuilder::new()
            .with_title("OpenGL Window")
            .with_inner_size(LogicalSize::new(800, 600));
        let template = ConfigTemplateBuilder::new().with_depth_size(24);

        // Get display
        let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (window, config) = display_builder
            .build(&event_loop, template, |configs| configs.next().unwrap())
            .unwrap();

        let window = window.unwrap();

        let window_handle = window.window_handle();
        let display = config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version::new(3, 3))))
            .build(Some(window_handle));

        let not_current_context: NotCurrentContext = unsafe {
            display
                .create_context(&config, &context_attributes)
                .expect("Failed to create OpenGL context")
        };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.window_handle(),
            window.inner_size().width,
            window.inner_size().height,
        );

        let surface = unsafe {
            display
                .create_window_surface(&config, &attrs)
                .expect("Failed to create surface")
        };

        let context = not_current_context
            .make_current(&surface)
            .expect("Failed to make context current");

        Ok(Self {
            window,
            context,
            surface,
            event_loop,
        })
    }

    fn poll_events(&mut self) -> Vec<Box<dyn crate::event::event::Event>> {
        let mut out_events = Vec::new();

        out_events 
    }

    fn get_size(&self) -> (u32, u32) {
        todo!()
    }

    fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    fn clear(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn resize(&self, width: i32, height: i32) {
        self.surface.resize(
            &self.context,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        );
    }
}
