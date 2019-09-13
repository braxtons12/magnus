use crate::events::event::*;
use crate::events::event::EventType::{KeyPressed, KeyReleased, TextInput};
use crate::events::event::EventCategory::{EventInput, EventKeyboard};
use crate::events::event::EventData::{U32p, I32p};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct KeyPressedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl KeyPressedEvent {
    pub fn new(message: String, keycode: i32, modifiers: i32) -> KeyPressedEvent {
        KeyPressedEvent { event_type: KeyPressed,
        category_flags: EventInput as u32 | EventKeyboard as u32,
        msg: message, data: I32p(keycode, modifiers), handled: false }
    }
}

impl std::fmt::Display for KeyPressedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyPressedEvent: (event_type: {}, category_flags: {}, 
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
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
    data: EventData,
    handled: bool
}

impl KeyReleasedEvent {
    pub fn new(message: String, keycode: i32, modifiers: i32) -> KeyReleasedEvent {
        KeyReleasedEvent { event_type: KeyReleased,
        category_flags: EventInput as u32 | EventKeyboard as u32,
        msg: message, data: I32p(keycode, modifiers), handled: false }
    }
}

impl std::fmt::Display for KeyReleasedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyReleasedEvent: (event_type: {}, category_flags: {}, 
        msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TextInputEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl TextInputEvent {
    pub fn new(message: String, keycode: u32, modifiers: u32) -> TextInputEvent {
        TextInputEvent { event_type: TextInput,
        category_flags: EventInput as u32 | EventKeyboard as u32,
        msg: message, data: U32p(keycode, modifiers), handled: false }
    }
}

impl std::fmt::Display for TextInputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextInputEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {}",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for TextInputEvent {

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
