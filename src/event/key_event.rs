use super::event::{Event, EventCategory, EventType};
use std::fmt;

// This module defines key events for a keyboard input system.
// Events:
// - KeyPressedEvent: Contains information about the key pressed, including its code and repeat count.
// - KeyReleasedEvent: Contains information about the key released, including its code.

#[derive(Debug)]
pub struct KeyPressedEvent {
    key_code: u32,
    repeat_count: u32,
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

    pub fn get_key_code(&self) -> u32 {
        self.key_code
    }

    pub fn get_repeat_count(&self) -> u32 {
        self.repeat_count
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

impl fmt::Display for KeyPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyPressedEvent: keycode={}, repeat={}, handled={}", 
        self.key_code, self.repeat_count, self.handled)
    }
}

#[derive(Debug)]
pub struct KeyReleasedEvent {
    pub key_code: u32,
    handled: bool,
}

impl KeyReleasedEvent {
    pub fn new(key_code: u32) -> Self {
        Self {
            key_code,
            handled: false,
        }
    }
}

impl Event for KeyReleasedEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::KeyPressed
    }

    fn name(&self) -> &str {
        "KeyReleased"
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

impl fmt::Display for KeyReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyReleasedEvent: keycode={}, handled={}", 
        self.key_code, self.handled)
    }
}