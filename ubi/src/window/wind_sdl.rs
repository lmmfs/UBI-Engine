use crate::ubiinfo;
use crate::event::event::Event::{WindowResize, WindowClose};
use crate::event::event_data::{WindowCloseEventData, WindowResizeEventData};
use crate::window::window_trait::{UBIWindow, WindowData};

use sdl2::{
    video::{GLContext, SwapInterval, Window},
    EventPump, Sdl,
};


pub struct SdlWindow {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl UBIWindow for SdlWindow {
    fn create(window_data: WindowData) -> Result<Self, String>
    where
        Self: Sized,
    {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // Add opengl atributes and context
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(
                window_data.name,
                window_data.width as u32,
                window_data.height as u32,
            )
            .resizable()
            .opengl()
            .build()
            .unwrap();

        ubiinfo!("Window created");

        // create opengl context
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        // set vsync
        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(SdlWindow {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }

    fn get_size(&self) -> (u32, u32) {
        self.window.size()
    }

    fn swap_buffers(&self) {
        self.window.gl_swap_window()
    }
    
    fn poll_events(&mut self) -> Vec<crate::event::event::Event> {
        let mut events: Vec<crate::event::event::Event> = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    events.push(WindowClose(WindowCloseEventData::default()));
                }

                sdl2::event::Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(width, height) => {
                            events.push(WindowResize(WindowResizeEventData::new(width, height)));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        events
    }
    
    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        }
    }
    
    fn resize(&self, width: i32, height: i32) {
        ubiinfo!("Resizing window to {}x{}", width, height);
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}
