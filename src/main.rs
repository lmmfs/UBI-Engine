use core::time;
use std::{f32::consts::PI, time::Instant};

use sdl2::event::{Event, WindowEvent};

mod windsdl;
use windsdl::Windsdl;
mod objects;
use objects::*;
mod transform;
use transform::*;

fn main() {
    let mut windsdl = Windsdl::new(800, 600).unwrap();
    unsafe { gl::Viewport(0, 0, 800, 600) }

    let program = create_program().unwrap();
    program.set();

    let (mut vertices, mut indices) = triangle_fan(3);


    let vbo = Vbo::generate();
    vbo.set(&vertices);

    let vao = Vao::generate();
    vao.set();

    let ibo = Ibo::generate();
    ibo.set(&indices);

    let mut model_matrix: Mat4 = Mat4::new();
    let mut view_matrix: Mat4 = Mat4::new();

    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    let u_time = Uniform::new(program.id(), "u_time").unwrap();
    let u_model = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view = Uniform::new(program.id(), "u_view_matrix").unwrap();

    unsafe { 
        gl::Uniform2f(u_resolution.id, 800., 600.);
        gl::Uniform1f(u_time.id, 0.0);
        gl::UniformMatrix4fv(u_model.id, 1, gl::TRUE, model_matrix.ptr());
        gl::UniformMatrix4fv(u_view.id, 1, gl::TRUE, view_matrix.ptr());
    }

    let start = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    'running: loop {
        for event in windsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, 
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                            gl::Uniform2f(u_resolution.id, width as f32, height as f32)
                        }
                    }
                }

                _ => { }, 
            }
        }

        unsafe {
            gl::ClearColor(20./255., 20./255., 20./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;

                (vertices, indices) = triangle_fan(seconds_elapsed % 6 + 3);
                vbo.set(&vertices);
                ibo.set(&indices);
            }
            model_matrix = Mat4::new();
            view_matrix = Mat4::new();

            let time_mod = start.elapsed().as_secs_f32() % 6.0;
            model_matrix.scale((time_mod+1.0)/5.0, (time_mod+1.0)/5.0, 0.0);
            model_matrix.rotate_z(time_mod.powi(3) / 2.);
            view_matrix.lookat(
                (time_mod / 3.0 * PI).sin()*0.5, 0.2, (time_mod / 3.0 * PI).cos()*0.5, 
                0.0, 0.0, 0.0, 
                0.0, 1.0, 0.0
            );

            gl::Uniform1f(u_time.id, start.elapsed().as_secs_f32());
            gl::UniformMatrix4fv(u_model.id, 1, gl::TRUE, model_matrix.ptr());
            gl::UniformMatrix4fv(u_view.id, 1, gl::TRUE, view_matrix.ptr());
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
