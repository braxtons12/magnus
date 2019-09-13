use crate::events::event::*;
use crate::events::event::EventType::
{MouseButtonPressed, MouseButtonReleased, MouseEntered, MouseMoved, MouseScrolled};
use crate::events::event::EventCategory::{EventInput, EventMouse, EventMouseButton};
use crate::events::event::EventData::{I32p, F32p};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseButtonPressedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl MouseButtonPressedEvent {
    pub fn new(message: String, button: i32, modifiers: i32) -> MouseButtonPressedEvent {
        MouseButtonPressedEvent { event_type: MouseButtonPressed,
        category_flags: EventInput as u32 | EventMouse as u32 | EventMouseButton as u32,
        msg: message, data: I32p(button, modifiers), handled: false }
    }
}

impl std::fmt::Display for MouseButtonPressedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseButtonPressedEvent(event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for MouseButtonPressedEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseButtonReleasedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl MouseButtonReleasedEvent {
    pub fn new(message: String, button:i32, modifiers: i32) -> MouseButtonReleasedEvent {
        MouseButtonReleasedEvent { event_type: MouseButtonReleased,
        category_flags: EventInput as u32 | EventMouse as u32 | EventMouseButton as u32,
        msg: message, data: I32p(button, modifiers), handled: false }
    }
}

impl std::fmt::Display for MouseButtonReleasedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseButtonReleasedEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for MouseButtonReleasedEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseEnteredEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl MouseEnteredEvent {
    pub fn new(message: String) -> MouseEnteredEvent {
        MouseEnteredEvent { event_type: MouseEntered,
        category_flags: EventInput as u32 | EventMouse as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for MouseEnteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseEnteredEvent: (event_type: {}, category_flags: {},
        msg: {}, handled: {}",
        self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for MouseEnteredEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        & self.msg
    }

    fn get_data(&self) -> Option<& EventData> {
        None
    }

    fn handled(&mut self) -> &mut bool {
        &mut self.handled
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseMovedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl MouseMovedEvent {
    pub fn new(message: String, x: f32, y: f32) -> MouseMovedEvent {
        MouseMovedEvent { event_type: MouseMoved,
        category_flags: EventInput as u32 | EventMouse as u32,
        msg: message, data: F32p(x, y), handled: false }
    }
}

impl std::fmt::Display for MouseMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseMovedEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for MouseMovedEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseScrolledEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl MouseScrolledEvent {
    pub fn new(message: String, x: f32, y: f32) -> MouseScrolledEvent {
        MouseScrolledEvent { event_type: MouseScrolled,
        category_flags: EventInput as u32 | EventMouse as u32,
        msg: message, data: F32p(x, y), handled: false }
    }
}

impl std::fmt::Display for MouseScrolledEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MouseScrolledEvent: (event_type: {}, category_flags: {}, 
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for MouseScrolledEvent {

    fn get_event_type(&self) -> EventType {
        self.event_type
    }

    fn get_category_flags(&self) -> u32 {
        self.category_flags
    }

    fn get_msg(&self) -> &String {
        &(self.msg)
    }

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }   
}
