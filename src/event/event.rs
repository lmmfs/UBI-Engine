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

pub struct EventDispatcher<'a> {
    event: &'a mut dyn Event,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a mut dyn Event) -> Self {
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
