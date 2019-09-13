use crate::events::event::*;
use crate::events::event::EventData::PathBufs;
use crate::events::event::EventType::{AppTick, AppUpdate, AppRender, AppFileDropped};
use crate::events::event::EventCategory::{EventApplication, EventInput};

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
        write!(f, "AppTickEvent: (event_type: {}, category_flags: {}, msg: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
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
        write!(f, "AppUpdateEvent: (event_type: {}, category_flags: {}, msg: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
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
        write!(f, "AppRenderEvent: (event_type: {}, category_flags: {}, msg: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
        None
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppFileDroppedEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl AppFileDroppedEvent {
    pub fn new(message: String, paths: Vec<std::path::PathBuf>) -> AppFileDroppedEvent {
        AppFileDroppedEvent { event_type: AppFileDropped,
        category_flags: EventApplication as u32 | EventInput as u32,
        msg: message, data: PathBufs(paths), handled: false }
    }
}

impl std::fmt::Display for AppFileDroppedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppFileDroppedEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {}",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for AppFileDroppedEvent {

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
