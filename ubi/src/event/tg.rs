use crate::event::application_event::WindowCloseEvent;
use crate::event::application_event::WindowResizeEvent;


enum EventLib {
    WindowClose(WindowCloseEvent),
    WindowResize(WindowResizeEvent),
}