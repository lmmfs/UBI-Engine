//use image::{imageops, DynamicImage, GenericImageView, ImageResult};

use gl::types::{GLint, GLsizei, GLuint};
use std::ffi::c_void;

pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn gen() -> Result<Self, String> {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Ok(Texture { id })
    }

    pub fn setup(&self, image_path: &str) {
        self.set_params();
        self.load_image_file(image_path).unwrap();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn set_params(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::MIRRORED_REPEAT as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::MIRRORED_REPEAT as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn load_image_file(&self, filepath: &str) -> Result<(), String> {
        let img = match image::open(filepath) {
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