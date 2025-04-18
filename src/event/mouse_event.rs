use super::event::{Event, EventCategory, EventType};
use std::fmt;

// This module contains mouse events for the mouse input system.
// Events:
// - MouseMovedEvent: Contains information about the mouse movement, including its position.
// - MouseButtonPressedEvent: Contains information about the mouse button pressed, including its code.
// - MouseButtonReleasedEvent: Contains information about the mouse button released, including its code.
// - MouseScrollEvent: Contains information about the mouse scroll, including its offset.

#[derive(Debug)]
pub struct MouseMovedEvent {
    x_pos: f32,
    y_pos: f32,
    handled: bool,
}

impl MouseMovedEvent{
    pub fn new(x_pos: f32, y_pos: f32) -> Self {
        Self { 
            x_pos: x_pos,
            y_pos: y_pos,
            handled: false 
        }
    }

    pub fn get_x_pos(&self) -> f32 {
        self.x_pos
    }

    pub fn get_y_pos(&self) -> f32 {
        self.y_pos
    }
}

impl Event for MouseMovedEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::MouseMoved
    }

    fn name(&self) -> &str {
        "MouseMoved"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::MOUSE
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

impl fmt::Display for MouseMovedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MouseMovedEvent: x={}, y={}, handled={}", 
        self.x_pos, self.y_pos, self.handled)
    }
}

#[derive(Debug)]
pub struct MouseButtonPressedEvent {
    button_code: u32,
    handled: bool,
}

impl MouseButtonPressedEvent{
    pub fn new(button_code: u32) -> Self {
        Self { 
            button_code: button_code,
            handled: false 
        }
    }

    pub fn get_button_code(&self) -> u32 {
        self.button_code
    }
}

impl Event for MouseButtonPressedEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::MouseButtonPressed
    }

    fn name(&self) -> &str {
        "MouseButtonPressed"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::INPUT | EventCategory::MOUSE | EventCategory::MOUSE_BUTTON
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

impl fmt::Display for MouseButtonPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MouseButtonPressedEvent: button_code={}, handled={}", 
        self.button_code, self.handled)
    }
}

#[derive(Debug)]
pub struct MouseButtonReleasedEvent {
    button_code: u32,
    handled: bool,
}

impl MouseButtonReleasedEvent{
    pub fn new(button_code: u32) -> Self {
        Self { 
            button_code: button_code,
            handled: false 
        }
    }

    pub fn get_button_code(&self) -> u32 {
        self.button_code
    }
}

impl Event for MouseButtonReleasedEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::MouseButtonReleased
    }

    fn name(&self) -> &str {
        "MouseButtonReleased"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::INPUT | EventCategory::MOUSE | EventCategory::MOUSE_BUTTON
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

impl fmt::Display for MouseButtonReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MouseButtonReleasedEvent: button_code={}, handled={}", 
        self.button_code, self.handled)
    }
}

#[derive(Debug)]
pub struct MouseScrollEvent {
    x_offset: f32,
    y_offset: f32,
    handled: bool,
}

impl MouseScrollEvent{
    pub fn new(x_offset: f32, y_offset: f32) -> Self {
        Self { 
            x_offset: x_offset,
            y_offset: y_offset,
            handled: false 
        }
    }

    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }

    pub fn get_y_offset(&self) -> f32 {
        self.y_offset
    }
}

impl Event for MouseScrollEvent {
    fn event_type(&self) -> EventType {
        Self::static_event_type()
    }

    fn static_event_type() -> EventType {
        EventType::MouseScroll
    }

    fn name(&self) -> &str {
        "MouseMoved"
    }

    fn category_flags(&self) -> EventCategory {
        EventCategory::MOUSE
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

impl fmt::Display for MouseScrollEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MouseScrollEvent: x={}, y={}, handled={}", 
        self.x_offset, self.y_offset, self.handled)
    }
}