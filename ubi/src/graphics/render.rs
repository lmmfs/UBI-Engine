use crate::core::custom_error::UbiError;
//use crate::core::logger::init;

use super::{buffer::{Ibo, Uniform, Vao, Vbo}, shader::create_program};
use super::shader::Program;

pub struct Renderer {
    pub program: Program,
    pub vao: Vao,
    pub vbo: Vbo,
    pub ibo: Ibo,
}

impl Renderer {
    pub fn new() -> Result<Self, UbiError> {
        let program = create_program().unwrap();
        program.set();
        let vbo = Vbo::gen();
        let vao = Vao::gen();
        vao.set();
        let ibo = Ibo::gen();

        Ok(Renderer{
            program: program,
            vao: vao,
            vbo: vbo,
            ibo: ibo,
        })
    }

    pub fn set_vertices() {

    }

    pub fn create_uniform(&self, name: &str) -> Result<(), UbiError> {
        Uniform::new(self.program.id(), name).unwrap();
        Ok(())
    }

    pub fn render() {
        /*
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
        */
    }
}
