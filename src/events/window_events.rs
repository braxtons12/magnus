use crate::events::event::*;
use crate::events::event::EventType::
{ WindowClose, WindowResize, WindowFocus, WindowMoved,
WindowRefresh, WindowIconify, WindowMaximize };
use crate::events::event::EventCategory::EventApplication;
use crate::events::event::EventData::{Bool, F32p};

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
        write!(f, "WindowCloseEvent: (event_type: {}, category_flags: {}, msg: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
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
    data: EventData,
    handled: bool
}

impl WindowResizeEvent {
    pub fn new(message: String, width: f32, height: f32) -> WindowResizeEvent {
        WindowResizeEvent { event_type: WindowResize,
        category_flags: EventApplication as u32,
        msg: message, data: F32p(width, height), handled: false }
    }
}

impl std::fmt::Display for WindowResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowResizeEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
       
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

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
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
    data: EventData,
    handled: bool
}

impl WindowFocusEvent {
    pub fn new(message: String, focused: bool) -> WindowFocusEvent {
        WindowFocusEvent { event_type: WindowFocus,
        category_flags: EventApplication as u32,
        msg: message, data: Bool(focused), handled: false }
    }
}

impl std::fmt::Display for WindowFocusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowFocusEvent: (event_type: {}, category_flags: {}, 
        msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
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
    data: EventData,
    handled: bool
}

impl WindowMovedEvent {
    pub fn new(message: String, x: f32, y: f32) -> WindowMovedEvent {
        WindowMovedEvent { event_type: WindowMoved,
        category_flags: EventApplication as u32,
        msg: message, data: F32p(x, y), handled: false }
    }
}

impl std::fmt::Display for WindowMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowMovedEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})", 
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
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

    fn get_data(&self) -> Option<& EventData> {
        Some(& self.data)
    }

    fn handled(&mut self) -> &mut bool {
        &mut(self.handled)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowRefreshEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    handled: bool
}

impl WindowRefreshEvent {
    pub fn new(message: String) -> WindowRefreshEvent {
        WindowRefreshEvent { event_type: WindowRefresh,
        category_flags: EventApplication as u32,
        msg: message, handled: false }
    }
}

impl std::fmt::Display for WindowRefreshEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowRefreshEvent: (event_type: {}, category_flags: {}, msg: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.handled)
    }
}

impl Event for WindowRefreshEvent {

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
pub struct WindowIconifyEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl WindowIconifyEvent {
    pub fn new(message: String, iconify: bool) -> WindowIconifyEvent {
        WindowIconifyEvent { event_type: WindowIconify,
        category_flags: EventApplication as u32,
        msg: message, data: Bool(iconify), handled: false }
    }
}

impl std::fmt::Display for WindowIconifyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowIconifyEvent: (event_type: {}, category_flags: {},
        msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for WindowIconifyEvent {

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
pub struct WindowMaximizeEvent {
    event_type: EventType,
    category_flags: u32,
    msg: String,
    data: EventData,
    handled: bool
}

impl WindowMaximizeEvent {
    pub fn new(message: String, maximize: bool) -> WindowMaximizeEvent {
        WindowMaximizeEvent { event_type: WindowMaximize,
        category_flags: EventApplication as u32,
        msg: message, data: Bool(maximize), handled: false }
    }
}

impl std::fmt::Display for WindowMaximizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowMaximizeEvent: (event_type: {}, category_flags: {}, 
        msg: {}, data: {}, handled: {})",
        self.event_type, self.category_flags, self.msg, self.data, self.handled)
    }
}

impl Event for WindowMaximizeEvent {

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
