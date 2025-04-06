use thiserror::Error;

#[derive(Error, Debug)]
pub enum UbiError {
    #[error("SDL2 error: {0}")]
    SdlError(#[from] sdl2::render::SdlError),

    #[error("OpenGL error: {0}")]
    GlError(String),

    #[error("Resource loading error: {0}")]
    ResourceError(String),

    #[error("Shader compilation error: {0}")]
    ShaderError(String),

    #[error("Window creation error: {0}")]
    WindowError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Model loading error: {0}")]
    ModelError(String),

    #[error("Texture loading error: {0}")]
    TextureError(String),

    #[error("ECS error: {0}")]
    EcsError(String),

    #[error("Other engine error: {0}")]
    Other(String),
}