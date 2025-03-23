use std::time::Instant;

use sdl2::event::Event;
use windsdl::Windsdl;

mod windsdl;

fn main() {
    println!("Hello, world!");
    let mut windsdl = Windsdl::new(800, 600).unwrap();

    let start = Instant::now();
     
    'running: loop {
        for event in windsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, 

                _ => { }, 
            }
        }

        unsafe {
            let red = (start.elapsed().as_secs_f32().sin() + 1.0) / 2.0;
            gl::ClearColor(red, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        windsdl.window.gl_swap_window();
    }
}
