use crate::event::event::Event;

pub trait Layer {
    fn on_attach(&mut self);
    fn on_detach(&mut self);
    fn on_update(&mut self, events: &mut Vec<Event>);
    fn on_event(&mut self, event: &mut Event);
}

// stack example 
// layer, layer, overlay
pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    overlay_start: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            overlay_start: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.overlay_start, layer);
        self.overlay_start += 1;
    }

    pub fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        if self.overlay_start > 0 {
            self.overlay_start -= 1;
            Some(self.layers.remove(self.overlay_start))
        } else {
            None
        }
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        if self.layers.len() > self.overlay_start {
            self.layers.pop()
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Layer>> {
        self.layers.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn Layer>> {
        self.layers.iter_mut()
    }
}
