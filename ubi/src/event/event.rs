use bitflags::bitflags;
use std::any::Any;
use std::fmt;

use super::event_data::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    KeyPressed,
    KeyReleased,
    MouseMoved,
    MouseScroll,
    MouseButtonPressed,
    MouseButtonReleased,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct EventCategory: u32 {
        const NONE        = 0;
        const APPLICATION = 1 << 0;
        const INPUT       = 1 << 1;
        const KEYBOARD    = 1 << 2;
        const MOUSE       = 1 << 3;
        const MOUSE_BUTTON = 1 << 4;
    }
}

#[derive(Debug)]
pub enum Event {
    WindowClose(WindowCloseEventData),
    WindowResize(WindowResizeEventData),
    KeyPressed(KeyPressedEventData),
    KeyReleased(KeyReleasedEventData),
    MouseMoved(MouseMovedEventData),
    MouseButtonPressed(MouseButtonPressedEventData),
    MouseButtonReleased(MouseButtonReleasedEventData),
    MouseScroll(MouseScrollEventData),
}

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Event::WindowClose(_) => EventType::WindowClose,
            Event::WindowResize(_) => EventType::WindowResize,
            Event::KeyPressed(_) => EventType::KeyPressed,
            Event::KeyReleased(_) => EventType::KeyReleased,
            Event::MouseMoved(_) => EventType::MouseMoved,
            Event::MouseButtonPressed(_) => EventType::MouseButtonPressed,
            Event::MouseButtonReleased(_) => EventType::MouseButtonReleased,
            Event::MouseScroll(_) => EventType::MouseScroll,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Event::WindowClose(_) => "WindowClose",
            Event::WindowResize(_) => "WindowResize",
            Event::KeyPressed(_) => "KeyPressed",
            Event::KeyReleased(_) => "KeyReleased",
            Event::MouseMoved(_) => "MouseMoved",
            Event::MouseButtonPressed(_) => "MouseButtonPressed",
            Event::MouseButtonReleased(_) => "MouseButtonReleased",
            Event::MouseScroll(_) => "MouseScroll",
        }
    }

    pub fn category_flags(&self) -> EventCategory {
        match self {
            Event::WindowClose(_) => EventCategory::APPLICATION,
            Event::WindowResize(_) => EventCategory::APPLICATION,
            Event::KeyPressed(_) => EventCategory::INPUT | EventCategory::KEYBOARD,
            Event::KeyReleased(_) => EventCategory::INPUT | EventCategory::KEYBOARD,
            Event::MouseMoved(_) => EventCategory::MOUSE,
            Event::MouseButtonPressed(_) => EventCategory::INPUT | EventCategory::MOUSE | EventCategory::MOUSE_BUTTON,
            Event::MouseButtonReleased(_) => EventCategory::INPUT | EventCategory::MOUSE | EventCategory::MOUSE_BUTTON,
            Event::MouseScroll(_) => EventCategory::MOUSE,
        }
    }

    pub fn is_in_category(&self, category: EventCategory) -> bool {
        self.category_flags().contains(category)
    }

    pub fn handled(&self) -> bool {
        match self {
            Event::WindowClose(data) => data.handled(),
            Event::WindowResize(data) => data.handled(),
            Event::KeyPressed(data) => data.handled(),
            Event::KeyReleased(data) => data.handled(),
            Event::MouseMoved(data) => data.handled(),
            Event::MouseButtonPressed(data) => data.handled(),
            Event::MouseButtonReleased(data) => data.handled(),
            Event::MouseScroll(data) => data.handled(),
        }
    }

    pub fn set_handled(&mut self, handled: bool) {
        match self {
            Event::WindowClose(data) => data.set_handled(handled),
            Event::WindowResize(data) => data.set_handled(handled),
            Event::KeyPressed(data) => data.set_handled(handled),
            Event::KeyReleased(data) => data.set_handled(handled),
            Event::MouseMoved(data) => data.set_handled(handled),
            Event::MouseButtonPressed(data) => data.set_handled(handled),
            Event::MouseButtonReleased(data) => data.set_handled(handled),
            Event::MouseScroll(data) => data.set_handled(handled),
        }
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::WindowClose(data) => write!(f, "WindowCloseEvent: handled={}", data.handled()),
            Event::WindowResize(data) => write!(
                f,
                "WindowResizeEvent: width={}, height={}, handled={}",
                data.get_width(),
                data.get_height(),
                data.handled()
            ),
            Event::KeyPressed(data) => write!(
                f,
                "KeyPressedEvent: keycode={}, repeat={}, handled={}",
                data.get_key_code(),
                data.get_repeat_count(),
                data.handled()
            ),
            Event::KeyReleased(data) => write!(
                f,
                "KeyReleasedEvent: keycode={}, handled={}",
                data.get_key_code(), data.handled()
            ),
            Event::MouseMoved(data) => write!(
                f,
                "MouseMovedEvent: x={}, y={}, handled={}",
                data.get_x_pos(), data.get_y_pos(), data.handled()
            ),
            Event::MouseButtonPressed(data) => write!(
                f,
                "MouseButtonPressedEvent: button_code={}, handled={}",
                data.get_button_code(), data.handled()
            ),
            Event::MouseButtonReleased(data) => write!(
                f,
                "MouseButtonReleasedEvent: button_code={}, handled={}",
                data.get_button_code(), data.handled()
            ),
            Event::MouseScroll(data) => write!(
                f,
                "MouseScrollEvent: x={}, y={}, handled={}",
                data.get_x_offset(), data.get_y_offset(), data.handled()
            ),
        }
    }
}

pub struct EventDispatcher {}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dispatch<F>(&mut self, event: &mut Event, mut func: F) -> bool
    where
        F: FnMut(&mut Event) -> bool,
    {
        let handled = func(event);
        event.set_handled(handled);
        handled
    }
}
