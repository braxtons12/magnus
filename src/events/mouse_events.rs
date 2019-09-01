use crate::events::event::*;
use crate::events::event::EventType::
{MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled};
use crate::events::event::EventCategory::{EventInput, EventMouse, EventMouseButton};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseButtonPressedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl MouseButtonPressedEvent {
    pub fn new(message: String, button: i32) -> MouseButtonPressedEvent {
        MouseButtonPressedEvent { event_type: MouseButtonPressed,
        category_flags: EventInput as u32 | EventMouse as u32 | EventMouseButton as u32,
        msg: message, data: vec![button as f32], handled: false }
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

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
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
    data: Vec<f32>,
    handled: bool
}

impl MouseButtonReleasedEvent {
    pub fn new(message: String, button:i32) -> MouseButtonReleasedEvent {
        MouseButtonReleasedEvent { event_type: MouseButtonReleased,
        category_flags: EventInput as u32 | EventMouse as u32 | EventMouseButton as u32,
        msg: message, data: vec![button as f32], handled: false }
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

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseMovedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl MouseMovedEvent {
    pub fn new(message: String, x: f32, y: f32) -> MouseMovedEvent {
        MouseMovedEvent { event_type: MouseMoved,
        category_flags: EventInput as u32 | EventMouse as u32,
        msg: message, data: vec![x, y], handled: false }
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

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
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
    data: Vec<f32>,
    handled: bool
}

impl MouseScrolledEvent {
    pub fn new(message: String, x: f32, y: f32) -> MouseScrolledEvent {
        MouseScrolledEvent { event_type: MouseScrolled,
        category_flags: EventInput as u32 | EventMouse as u32,
        msg: message, data: vec![x, y], handled: false }
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

    fn get_data(&self) -> Option<& Vec<f32>> {
        Some(&(self.data))
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }   
}

