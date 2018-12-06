use crate::events::event::Event;
use crate::events::event::EventType;
use crate::events::event::EventCategory;

pub struct WindowResizeEvent {
    handled: bool,
    width: u32,
    height: u32
}

impl WindowResizeEvent {
    fn new(width: u32, height: u32) -> WindowResizeEvent {
        WindowResizeEvent { handled: false, width: width, height: height }
    }

    fn get_width(&self) -> u32 { self.width }
    fn get_height(&self) -> u32 { self.height }
}

impl Event for WindowResizeEvent {

    fn get_static_type(&self) -> EventType {
        EventType::WindowResize
    }

    fn get_event_type(&self) -> EventType {
        WindowResizeEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("WindowResize")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventApplication as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set; }

    fn box_clone(&self) -> Box<Event> {
        let x = WindowResizeEvent { handled: self.handled, width: self.width, height: self.height };
        Box::new(x)
    }
}

pub struct AppTickEvent {
    handled: bool
}

impl Event for AppTickEvent {
    fn get_static_type(&self) -> EventType {
        EventType::AppTick
    }

    fn get_event_type(&self) -> EventType {
        AppTickEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("AppTick")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventApplication as i32
    }

    fn is_handled(&self) -> bool { self.handled }
    fn set_handled(&mut self, set: bool) { self.handled = set; }

    fn box_clone(&self) -> Box<Event> {
        let x = AppTickEvent { handled: self.handled };
        Box::new(x)
    }
}


pub struct AppUpdateEvent {
    handled: bool
}

impl Event for AppUpdateEvent {
    fn get_static_type(&self) -> EventType {
        EventType::AppUpdate
    }

    fn get_event_type(&self) -> EventType {
        AppUpdateEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("AppUpdate")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventApplication as i32
    }

    fn is_handled(&self) -> bool { self.handled }
    fn set_handled(&mut self, set: bool) { self.handled = set; }

    fn box_clone(&self) -> Box<Event> {
        let x = AppUpdateEvent { handled: self.handled };
        Box::new(x)
    }
}

pub struct AppRenderEvent {
    handled: bool
}

impl Event for AppRenderEvent {
    fn get_static_type(&self) -> EventType {
        EventType::AppRender
    }

    fn get_event_type(&self) -> EventType {
        AppRenderEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("AppRender")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventApplication as i32
    }

    fn is_handled(&self) -> bool { self.handled }
    fn set_handled(&mut self, set: bool) { self.handled = set; }

    fn box_clone(&self) -> Box<Event> {
        let x = AppRenderEvent { handled: self.handled };
        Box::new(x)
    }
}
