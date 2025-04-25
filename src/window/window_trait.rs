pub struct WindowData<'a> {
    pub name: &'a str,
    pub width: usize,
    pub height: usize,
}

pub trait UBIWindow {
    fn create(window_data: WindowData) -> Result<Self, String>
    where
        Self: Sized;
    fn poll_events(&mut self) -> Vec<Box<dyn crate::event::event::Event>>;
    fn get_size(&self) -> (u32, u32);
    fn swap_buffers(&self);
    fn clear(&self);
    fn resize(&self, width: i32, height: i32);
}
