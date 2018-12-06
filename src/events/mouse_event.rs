use crate::events::event::Event;
use crate::events::event::EventType;
use crate::events::event::EventCategory;

pub struct MouseMovedEvent {
    handled: bool,
    mouse_x: f32,
    mouse_y: f32
}

impl MouseMovedEvent {
    fn new(x: f32, y: f32) -> MouseMovedEvent {
        MouseMovedEvent { handled: false, mouse_x: x, mouse_y: y }
    }

    #[inline(always)]
    fn get_x(&self) -> f32 { self.mouse_x }

    #[inline(always)]
    fn get_y(&self) -> f32 { self.mouse_y }
}

impl Event for MouseMovedEvent {

    fn get_static_type(&self) -> EventType {
        EventType::MouseMoved
    }

    fn get_event_type(&self) -> EventType {
        MouseMovedEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("MouseMoved")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set }

    fn box_clone(&self) -> Box<Event> {
        let x = MouseMovedEvent { handled: self.handled, mouse_x: self.mouse_x, mouse_y: self.mouse_y };
        Box::new(x)
    }
} 
