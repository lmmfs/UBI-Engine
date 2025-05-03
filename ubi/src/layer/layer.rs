use crate::event::event::Event;

pub trait Layer {
   fn new() -> Self;
   fn on_attach(&self);
   fn on_dettach(&self);
   fn on_update(&self);
   fn on_event(&self, event: Event);
}