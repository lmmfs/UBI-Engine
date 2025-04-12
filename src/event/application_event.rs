use super::event::{Event, EventCategory, EventType};

#[derive(Debug)]
pub struct WindowCloseEvent {
    handled: bool,
}

impl WindowCloseEvent {
    pub fn new() -> Self {
        Self { handled: false }
    }
}

impl Event for WindowCloseEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::WindowClose
    }

    fn name(&self) -> &str {
        "WindowClose"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
}



#[derive(Debug)]
pub struct WindowResizeEvent {
    pub width: i32, 
    pub height: i32,
    handled: bool,
}

impl WindowResizeEvent {
    pub fn new( width: i32,  height: i32) -> Self {
        Self { 
            width: width,
            height: height,
            handled: false 
        }
    }
}

impl Event for WindowResizeEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::WindowResize
    }

    fn name(&self) -> &str {
        "WindowResize"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::APPLICATION
    }

    fn handled(&self) -> bool {
        self.handled
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}