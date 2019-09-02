use crate::events::event::*;
use crate::events::event::EventType::
{WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved};
use crate::events::event::EventCategory::EventApplication;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowCloseEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl WindowCloseEvent {
    pub fn new(message: String) -> WindowCloseEvent {
        WindowCloseEvent { event_type: WindowClose,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for WindowCloseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowCloseEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& Vec<f32>> {
        None
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowResizeEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl WindowResizeEvent {
    pub fn new(message: String, width: f32, height: f32) -> WindowResizeEvent {
        WindowResizeEvent { event_type: WindowResize,
        category_flags: EventApplication as u32,
        msg: message, data: vec![width, height], handled: false }
    }
}

impl std::fmt::Display for WindowResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowResizeEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowFocusEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl WindowFocusEvent {
    pub fn new(message: String) -> WindowFocusEvent {
        WindowFocusEvent { event_type: WindowFocus,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for WindowFocusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowFocusEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& Vec<f32>> {
        None
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowLostFocusEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl WindowLostFocusEvent {
    pub fn new(message: String) -> WindowLostFocusEvent {
        WindowLostFocusEvent { event_type: WindowLostFocus,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for WindowLostFocusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowLostFocusEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& Vec<f32>> {
        None
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowMovedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl WindowMovedEvent {
    pub fn new(message: String, x: f32, y: f32) -> WindowMovedEvent {
        WindowMovedEvent { event_type: WindowMoved,
        category_flags: EventApplication as u32,
        msg: message, data: vec![x, y], handled: false }
    }
}

impl std::fmt::Display for WindowMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowMovedEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}
