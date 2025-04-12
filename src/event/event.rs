use std::any::Any;
use std::fmt;

bitflags::bitflags! {
    pub struct EventCategory: u32 {
        const NONE        = 0;
        const APPLICATION = 1 << 0;
        const INPUT       = 1 << 1;
        const KEYBOARD    = 1 << 2;
        const MOUSE       = 1 << 3;
        const MOUSE_BUTTON = 1 << 4;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            EventType::KeyPressed => write!(f, "KeyPressed"),
            EventType::KeyReleased => write!(f, "KeyReleased"),
            EventType::MouseMoved => write!(f, "MouseMoved"),
            EventType::MouseScroll => write!(f, "MouseScroll"),
            EventType::MouseButtonPressed => write!(f, "MouseButtonPressed"),
            EventType::MouseButtonReleased => write!(f, "MouseButtonReleased"),
            EventType::WindowClose => write!(f, "WindowClose"),
            EventType::WindowResize => write!(f, "WindowResize"),
            EventType::WindowFocus => write!(f, "WindowFocus"),
            EventType::WindowLostFocus => write!(f, "WindowLostFocus"),
            EventType::WindowMoved => write!(f, "WindowMoved"), 
       }
    }
}

pub trait Event: fmt::Debug + Any {
    fn event_type(&self) -> EventType;
    fn static_event_type() -> EventType
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn category_flags(&self) -> EventCategory;
    fn is_in_category(&self, category: EventCategory) -> bool {
        self.category_flags().contains(category)
    }
    fn handled(&self) -> bool;
    fn set_handled(&mut self, handled: bool);
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct EventDispatcher {
    event: Box<dyn Event>,
}

impl EventDispatcher {
    pub fn new(event: Box<dyn Event>) -> Self {
        Self { event }
    }

    pub fn dispatch<E: Event + 'static, F>(&mut self, mut func: F) -> bool
    where
        F: FnMut(&mut E) -> bool,
    {
        if self.event.event_type() == E::static_event_type() {
            if let Some(e) = self.event.as_any().downcast_mut::<E>() {
                let handled = func(e);
                self.event.set_handled(handled);
                return true;
            }
        }
        false
    }
}
