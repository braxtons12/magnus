use crate::events::event::Event;
use crate::events::event::EventType;
use crate::events::event::EventCategory;

pub struct KeyPressedEvent {
    handled: bool,
    keycode: i32,
    repeat_count: i32
}

impl KeyPressedEvent {
    fn new(keycode: i32, repeat_count: i32) -> KeyPressedEvent {
        KeyPressedEvent { handled: false, keycode: keycode, repeat_count: repeat_count } 
    }

    #[inline(always)]
    fn get_keycode(&self) -> i32 { self.keycode }
    #[inline(always)]
    fn get_repeat_count(&self) -> i32 { self.repeat_count }
}

impl Event for KeyPressedEvent {

    fn get_static_type(&self) -> EventType {
        EventType::KeyPressed
    }

    fn get_event_type(&self) -> EventType {
        KeyPressedEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("KeyPressed")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set; }

    fn box_clone(&self) -> Box<Event> {
        let x = KeyPressedEvent { handled: self.handled, keycode: self.keycode, 
            repeat_count: self.repeat_count };
        Box::new(x)
    }
}

pub struct KeyReleasedEvent {
    handled: bool,
    keycode: i32
}

impl KeyReleasedEvent {
    fn new(keycode: i32) -> KeyReleasedEvent {
        KeyReleasedEvent { handled: false, keycode: keycode }
    }

    #[inline(always)]
    fn get_keycode(&self) -> i32 { self.keycode }
}

impl Event for KeyReleasedEvent {

    fn get_static_type(&self) -> EventType {
        EventType::KeyReleased
    }

    fn get_event_type(&self) -> EventType {
        KeyReleasedEvent::get_static_type(self)
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn to_string(&self) -> String {
        String::from("KeyReleased")
    }

    fn get_category_flags(&self) -> i32 {
        EventCategory::EventInput as i32
    }

    fn is_handled(&self) -> bool { self.handled }

    fn set_handled(&mut self, set: bool) { self.handled = set }

    fn box_clone(&self) -> Box<Event> {
        let x = KeyReleasedEvent { handled: self.handled, keycode: self.keycode };
        Box::new(x)
    }
}
