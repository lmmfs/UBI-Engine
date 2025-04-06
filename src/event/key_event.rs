use super::event::{Event, EventCategory, EventType};

#[derive(Debug)]
pub struct KeyPressedEvent {
    pub key_code: u32,
    pub repeat_count: u32,
    handled: bool,
}

impl KeyPressedEvent {
    pub fn new(key_code: u32, repeat_count: u32) -> Self {
        Self {
            key_code,
            repeat_count,
            handled: false,
        }
    }
}

impl Event for KeyPressedEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::KeyPressed
    }

    fn name(&self) -> &str {
        "KeyPressed"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::INPUT | EventCategory::KEYBOARD
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
