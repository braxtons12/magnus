use crate::events::event::*;
use crate::events::event::EventType::
{RenderFramebufferResize, RenderContentScaleResize};
use crate::events::event::EventCategory::EventApplication;
use crate::events::event::EventData::F32p;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RenderFramebufferResizeEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl RenderFramebufferResizeEvent {
    pub fn new(message: String, width: f32, height: f32) -> RenderFramebufferResizeEvent {
        RenderFramebufferResizeEvent { event_type: RenderFramebufferResize,
        category_flags: EventApplication as u32,
        msg: message, data: F32p(width, height), handled: false }
    }
}

impl std::fmt::Display for RenderFramebufferResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderFramebufferResizeEvent: (event_type: {}, 
        category_flags: {}, msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for RenderFramebufferResizeEvent {

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
pub struct RenderContentScaleResizeEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl RenderContentScaleResizeEvent {
    pub fn new(message: String, width: f32, height: f32) -> RenderContentScaleResizeEvent {
        RenderContentScaleResizeEvent { event_type: RenderContentScaleResize,
        category_flags: EventApplication as u32,
        msg: message, data: F32p(width, height), handled: false }
    }
}

impl std::fmt::Display for RenderContentScaleResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderContentScaleResizeEvent: (event_type: {},
        category_flags: {}, msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for RenderContentScaleResizeEvent {

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