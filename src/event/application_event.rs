use super::event::{Event, EventCategory, EventType};
use std::fmt;

// This module contains event types related to application events, such as closing and resizing.
// Events:
// - WindowCloseEvent: Represents a request to close the application window.
// - WindowResizeEvent: Represents a request to resize the application window.

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

impl fmt::Display for WindowCloseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WindowCloseEvent: handled={}", self.handled)
    }
}


#[derive(Debug)]
pub struct WindowResizeEvent {
    width: i32, 
    height: i32,
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

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
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

impl fmt::Display for WindowResizeEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WindowCloseEvent: width={}, height={}, handled={}", 
        self.width, self.height, self.handled)
    }
}