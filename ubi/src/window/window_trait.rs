use crate::core::custom_error::UbiError;

pub struct WindowData<'a> {
    pub name: &'a str,
    pub width: usize,
    pub height: usize,
}

impl Default for WindowData<'_> {
    fn default() -> Self {
        Self {
            name: "UBI",
            width: 1280,
            height: 720,
        }
    }
}

pub trait UBIWindow {
    fn create(window_data: WindowData) -> Result<Self, UbiError>
    where
        Self: Sized;
    fn poll_events(&mut self, events: &mut Vec<crate::event::event::Event>) -> Result<(), UbiError>;
    fn get_size(&self) -> (u32, u32);
    fn swap_buffers(&self);
    fn clear(&self);
    fn resize(&self, width: i32, height: i32);
}
