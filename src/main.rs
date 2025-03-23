use std::{f32::consts::PI, time::Instant};

use sdl2::event::Event;

mod windsdl;
use windsdl::Windsdl;
mod objects;
use objects::*;

fn main() {
    println!("Hello, world!");
    let mut windsdl = Windsdl::new(800, 600).unwrap();

    let program = create_program().unwrap();
    program.set();

    let (mut vertices, mut indices) = triangle_fan(3);


    let vbo = Vbo::generate();
    vbo.set(&vertices);

    let vao = Vao::generate();
    vao.set();

    let ibo = Ibo::generate();
    ibo.set(&indices);

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    'running: loop {
        for event in windsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, 

                _ => { }, 
            }
        }

        unsafe {
            gl::ClearColor(0.5, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = triangle_fan(seconds_elapsed + 3);
                println!("{:?}\n{:?}", vertices, indices);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

            gl::DrawElements(
                gl::TRIANGLES, 
                indices.len() as i32, 
                gl::UNSIGNED_INT, 
                0 as *const _
            );
        }

        windsdl.window.gl_swap_window();
    }
}


fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = vec![
        0.0, 0.0,
        0.5, 0.0,
    ];
    let mut indices: Vec<u32> = vec![];

    let mut angle:f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);

        indices.push(0);
        indices.push(m);
        indices.push(m+1);
    }

    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}
