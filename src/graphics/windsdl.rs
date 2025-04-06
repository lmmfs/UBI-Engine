use sdl2::{video::{gl_attr, GLContext, SwapInterval, Window}, EventPump, Sdl};
use crate::ubiinfo;


pub struct Windsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl Windsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // Add opengl atributes and context
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3); 

        let window = video_subsystem
            .window("rusty", width as u32, height as u32)
            .resizable()
            .opengl()
            .build()
            .unwrap();

        // create opengl context
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with( |s | {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        // set vsync
        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(Windsdl {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })  
    }
}