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

    let (mut vertices, mut indices) = gem_triangle();
    
    let vbo = Vbo::gen();
    vbo.set(&vertices);
    

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    let texture: Texture = Texture::gen().unwrap();
    texture.setup("assets/wall.jpg");

    let mut model_matrix: Mat4 = Mat4::new();
    let mut view_matrix: Mat4 = Mat4::new();

    let u_time = Uniform::new(program.id(), "u_time").unwrap();
    let u_model = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_texture = Uniform::new(program.id(), "u_texture").unwrap();

    unsafe { 
        gl::Uniform1f(u_time.id, 0.0);
        gl::UniformMatrix4fv(u_model.id, 1, gl::TRUE, model_matrix.ptr());
        gl::UniformMatrix4fv(u_view.id, 1, gl::TRUE, view_matrix.ptr());
    }

    let start = Instant::now();

    'running: loop {
        for event in windsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running, 
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                        }
                    }
                }

                _ => { }, 
            }
        }

        unsafe {
            gl::ClearColor(20./255., 20./255., 20./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();
            gl::Uniform1i(u_texture.id, 0);

            (vertices, indices) = gem_triangle();
            vbo.set(&vertices);
            ibo.set(&indices);

            model_matrix = Mat4::new();
            view_matrix = Mat4::new();

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


fn gem_triangle() -> (Vec<f32>, Vec<u32>) {
    //vertice: x, y, z,  uv.x, uv.y 
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,   0.0, 0.0, // bottom left
        0.5 , -0.5, 0.0,   1.0, 0.0,  // bottom right
        0.0 ,  0.5, 0.0,   0.5, 1.0,   // top  
         
    ];
    let indices: Vec<u32> = vec![
        0, 1, 2,
    ];

    (vertices, indices)
}