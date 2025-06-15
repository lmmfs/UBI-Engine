// This module contains event types related to application events, such as closing and resizing.

// - WindowCloseEvent: Represents a request to close the application window.
#[derive(Debug, Default, Copy, Clone)]
pub struct WindowCloseEventData {
    handled: bool,
}

impl WindowCloseEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }
}

// - WindowResizeEvent: Represents a request to resize the application window.
#[derive(Debug, Copy, Clone)]
pub struct WindowResizeEventData {
    width: i32,
    height: i32,
    handled: bool,
}

impl WindowResizeEventData {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            handled: false,
        }
    }

    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }
}

// - KeyPressedEvent: Contains information about the key pressed, including its code and repeat count.
#[derive(Debug)]
pub struct KeyPressedEventData {
    key_code: u32,
    repeat_count: u32,
    handled: bool,
}

impl KeyPressedEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_key_code(&self) -> u32 {
        self.key_code
    }

    pub fn get_repeat_count(&self) -> u32 {
        self.repeat_count
    }
}

// - KeyReleasedEvent: Contains information about the key released, including its code.
#[derive(Debug)]
pub struct KeyReleasedEventData {
    key_code: u32,
    handled: bool,
}

impl KeyReleasedEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_key_code(&self) -> u32 {
        self.key_code
    }
}

// - MouseMovedEvent: Contains information about the mouse movement, including its position.
#[derive(Debug)]
pub struct MouseMovedEventData {
    x_pos: f32,
    y_pos: f32,
    handled: bool,
}

impl MouseMovedEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_x_pos(&self) -> f32 {
        self.x_pos
    }

    pub fn get_y_pos(&self) -> f32 {
        self.y_pos
    }
}

// - MouseButtonPressedEvent: Contains information about the mouse button pressed, including its code.
#[derive(Debug)]
pub struct MouseButtonPressedEventData {
    button_code: u32,
    x: i32,
    y: i32,
    clicks: u32,
    handled: bool,
}

impl MouseButtonPressedEventData {
    pub fn new(button_code: u32, x: i32, y: i32, clicks: u32) -> Self {
        Self {
            button_code: button_code,
            x: x,
            y: y,
            clicks: clicks,
            handled: false,
        }
    }

    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_button_code(&self) -> u32 {
        self.button_code
    }

    pub fn get_button_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// - MouseButtonReleasedEvent: Contains information about the mouse button released, including its code.
#[derive(Debug)]
pub struct MouseButtonReleasedEventData {
    button_code: u32,
    handled: bool,
}

impl MouseButtonReleasedEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_button_code(&self) -> u32 {
        self.button_code
    }
}

// - MouseScrollEvent: Contains information about the mouse scroll, including its offset.
#[derive(Debug)]
pub struct MouseScrollEventData {
    x_offset: f32,
    y_offset: f32,
    handled: bool,
}

impl MouseScrollEventData {
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }

    pub fn get_y_offset(&self) -> f32 {
        self.y_offset
    }
}
