use crate::events::event::*;
use crate::events::event::EventType::{AppTick, AppUpdate, AppRender};
use crate::events::event::EventCategory::EventApplication;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppTickEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl AppTickEvent {

    pub fn new(message: String) -> AppTickEvent {
        AppTickEvent { event_type: AppTick, 
            category_flags: EventApplication as u32, 
            msg: message, handled: false }
    }
}

impl std::fmt::Display for AppTickEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for AppTickEvent {

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
pub struct AppUpdateEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl AppUpdateEvent {

    pub fn new(message: String) -> AppUpdateEvent {
        AppUpdateEvent { event_type: AppUpdate,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for AppUpdateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for AppUpdateEvent {

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
pub struct AppRenderEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl AppRenderEvent {
    pub fn new(message: String) -> AppRenderEvent {
        AppRenderEvent { event_type: AppRender,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for AppRenderEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for AppRenderEvent {

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
