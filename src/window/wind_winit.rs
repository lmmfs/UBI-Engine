#![allow(unused)]
#[allow(dead_code)]
//glutin imports
use glutin::{
    config::{Config, ConfigTemplateBuilder, GetGlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
    },
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SwapInterval, WindowSurface},
};

use raw_window_handle::HasWindowHandle;
use winit::event_loop::EventLoop;

// ubi imports
use crate::event::application_event::*;
use crate::ubiinfo;
use crate::window::window_trait::{UBIWindow, WindowData};

pub struct WinitWindow {
    /*
    window: winit::window::Window,
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    context: glutin::context::PossiblyCurrentContext,
    gl_display: glutin::display::Display,
    event_loop: winit::event_loop::EventLoop<()>, // will use run_return()
    event_buffer: Vec<winit::event::Event<()>>,
    */
}

impl UBIWindow for WinitWindow {
    fn create(window_data: crate::prelude::WindowData) -> Result<Self, String>
    where
        Self: Sized,
    {
        let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
        Ok(Self {
        })
    }

    fn poll_events(&mut self) -> Vec<Box<dyn crate::event::event::Event>> {
        todo!()
    }

    fn get_size(&self) -> (u32, u32) {
        todo!()
    }

    fn swap_buffers(&self) {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }

    fn resize(&self, width: i32, height: i32) {
        todo!()
    }
}
