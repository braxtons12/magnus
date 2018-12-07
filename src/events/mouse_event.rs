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

pub struct MouseScrolledEvent {
    handled: bool,
    x_offset: f32,
    y_offset: f32
}

impl MouseScrolledEvent {
    fn new(x: f32, y: f32) -> MouseScrolledEvent {
        MouseScrolledEvent { handled: false, x_offset: x, y_offset: y }
    }

    #[inline(always)]
    fn get_x_offset(&self) -> f32 { self.x_offset }

    #[inline(always)]
    fn get_y_offset(&self) -> f32 { self.y_offset }
}

impl Event for MouseScrolledEvent {

    fn get_static_type(&self) -> EventType {
        EventType::MouseScrolled
    }

    fn get_event_type(&self) -> EventType {
        MouseScrolledEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("MouseScrolled")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set }

    fn box_clone(&self) -> Box<Event> {
        let x = MouseScrolledEvent { handled: self.handled, x_offset: self.x_offset,
        y_offset: self.y_offset };
        Box::new(x)
    }
}

pub struct MouseButtonPressedEvent {
    handled: bool,
    button: i32
}

impl MouseButtonPressedEvent {
    fn new(button: i32) -> MouseButtonPressedEvent {
        MouseButtonPressedEvent { handled: false, button: button }
    }

    fn get_button(&self) -> i32 { self.button }
}

impl Event for MouseButtonPressedEvent {

    fn get_static_type(&self) -> EventType {
        EventType::MouseButtonPressed
    }

    fn get_event_type(&self) -> EventType {
        MouseButtonPressedEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("MouseButtonPressed")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set }

    fn box_clone(&self) -> Box<Event> {
        let x = MouseButtonPressedEvent { handled: self.handled, button: self.button };
        Box::new(x)
    }
}

pub struct MouseButtonReleasedEvent {
    handled: bool,
    button: i32
}

impl MouseButtonReleasedEvent {
    fn new(button: i32) -> MouseButtonReleasedEvent {
        MouseButtonReleasedEvent { handled: false, button: button }
    }

    fn get_button(&self) -> i32 { self.button }
}

impl Event for MouseButtonReleasedEvent {

    fn get_static_type(&self) -> EventType {
        EventType::MouseButtonReleased
    }

    fn get_event_type(&self) -> EventType {
        MouseButtonReleasedEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("MouseButtonReleased")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set }

    fn box_clone(&self) -> Box<Event> {
        let x = MouseButtonReleasedEvent { handled: self.handled, button: self.button };
        Box::new(x)
    }
}
