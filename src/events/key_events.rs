use crate::events::event::*;
use crate::events::event::EventType::{KeyPressed, KeyReleased};
use crate::events::event::EventCategory::{EventInput, EventKeyboard};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct KeyPressedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl KeyPressedEvent {
    pub fn new(message: String, keycode: i32) -> KeyPressedEvent {
        KeyPressedEvent { event_type: KeyPressed,
        category_flags: EventInput as u32 | EventKeyboard as u32,
        msg: message, data: vec![keycode as f32], handled: false }
    }
}

impl Event for KeyPressedEvent {

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
pub struct KeyReleasedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: Vec<f32>,
    handled: bool
}

impl KeyReleasedEvent {
    pub fn new(message: String, keycode: i32) -> KeyReleasedEvent {
        KeyReleasedEvent { event_type: KeyReleased,
        category_flags: EventInput as u32 | EventKeyboard as u32,
        msg: message, data: vec![keycode as f32], handled: false }
    }
}

impl Event for KeyReleasedEvent {

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
