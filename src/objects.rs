use std::{ffi::{c_void, CStr, CString}, mem, ptr::{null, null_mut}};

use gl::{types::{GLchar, GLenum, GLint, GLuint, GLsizei}, DeleteBuffers};

use image::{imageops, DynamicImage, GenericImageView, ImageResult};

// An OpenGL Shader
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }

        // check if shader return error
        if success == 0 {
            let mut len: GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitspace_cstring_with_len(len as usize);

            unsafe { 
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

// An OpenGL Program, a sequence off shader calls
pub struct Program {
    id: GLuint,
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success: GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::COMPILE_STATUS, &mut success); }

        // check if program return error
        if success == 0 {
            let mut len: GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitspace_cstring_with_len(len as usize);

            unsafe { 
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(Program { id })

    }

    pub fn set(&self) {
        unsafe { gl::UseProgram(self.id);}
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}


impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

fn create_whitspace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_program() -> Result<Program, &'static str> {
    let vert_shader = Shader::from_source(&CString::new(include_str!(".vert")).unwrap(), gl::VERTEX_SHADER).unwrap(); 
    let frag_shader = Shader::from_source(&CString::new(include_str!(".frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();
    check_gl_error();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    Ok(shader_program)
}

// OpenGL Vertex Buffer Obejct
pub struct Vbo {
    id: GLuint,
}

impl Vbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Vbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }

    fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}


// OpenGL Index Buffer Obejct
pub struct Ibo {
    id: GLuint,
}

impl Ibo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Ibo { id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW
            );
        }
    }

    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }

    fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for Ibo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}


pub struct Vao {
    id: GLuint,
}

impl Vao {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        unsafe {
           gl::EnableVertexAttribArray(0);
           gl::VertexAttribPointer(
                0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                (5 * std::mem::size_of::<f32>()) as GLint, 
                null()
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, 
                2, 
                gl::FLOAT, 
                gl::FALSE, 
                (5 * std::mem::size_of::<f32>()) as GLint, 
                (3 * mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            
        }
    }

    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }

    fn delete(&self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: u32, name: &str) -> Result<Self, String> {
        let cname: CString = CString::new(name).expect("CString::new failed");
        let location: GLint = unsafe { gl::GetUniformLocation(program, cname.as_ptr()) };

        if location == -1 {
            return Err(format!("Couldn't get uniform location for {}", name));
        }

        Ok(Uniform { id: location})
    }
}

pub struct Texture {
    pub id: GLuint,
}

impl Texture {

    pub fn gen() -> Result<Self, String> {
        let mut id: GLuint = 0;
        unsafe { gl::GenTextures(1, &mut id); }
        Ok(Texture { id })
    }

    pub fn setup(&self, image_path: &str) {
        self.set_params();
        self.load_image_file(image_path).unwrap();
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }

    fn set_params(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0); 
        }
    }

    fn load_image_file(&self, filepath: &str) -> Result<(), String> {
        let mut img  = match image::open(filepath) {
            Ok(img) => img.to_rgb8(),
            Err(e) => return Err(format!("Failed to load image: {}", e)),
        };
        
        let img = image::imageops::flip_vertical(&img);

        let (width, height) = img.dimensions();
        let data = img.into_raw();

        println!("Loaded texture dimensions: {}x{}", width, height);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(())
    }

}

fn check_gl_error() {
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            eprintln!("OpenGL error: {}", error);
            panic!("OpenGL error occurred!");
        }
    }
}


