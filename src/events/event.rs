use std::boxed::Box;
use std::clone::Clone;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EventType {
    None = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EventCategory {
    None = 0,
    EventApplication    = BIT!(0),
    EventInput          = BIT!(1),
    EventKeyboard       = BIT!(2),
    EventMouse          = BIT!(3),
    EventMouseButton    = BIT!(4)
}

//impl struct must have a handled: bool
pub trait Event {
    fn get_static_type(&self) -> EventType;
    fn get_event_type(&self) -> EventType;

    #[cfg(debug_assertions)]
    fn get_name(&self) -> String;
    #[cfg(debug_assertions)]
    fn to_string(&self) -> String;

    fn get_category_flags(&self) -> i32;

    #[inline(always)]
    fn is_handled(&self) -> bool;

    #[inline(always)]
    fn set_handled(&mut self, set: bool);

    #[inline(always)]
    fn is_in_category(&self, category: EventCategory) -> bool {
        if self.get_category_flags() & category as i32 > 0 { true } else { false }
    }

    fn box_clone(&self) -> Box<Event>;
}

impl Clone for Box<Event> {
    fn clone(&self) -> Box<Event> {
        self.box_clone()
    }
}
pub struct EventDispatcher {
    event: Box<Event>
}

impl EventDispatcher {

    fn new(event: Box<Event>) -> EventDispatcher {
        EventDispatcher { event: event }
    }

    fn dispatch<T>(&mut self, func: fn(&Event) -> bool) -> bool {
        unsafe {
            let x = self.event.clone();
            let y = self.event.clone();
            let mut x = Box::into_raw(x);
            let mut y = Box::into_raw(y);
            if self.event.as_mut().get_event_type() == Event::get_static_type(&*x) {
                let z = func(&*y);
                self.event = Box::from_raw(y);
                self.event.set_handled(z);
                return true;
            }
            false
        }
    }
}
