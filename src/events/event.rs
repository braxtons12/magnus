#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum EventType {
    None = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum EventCategory {
    None = 0,
    EventApplication    = BIT!(0),
    EventInput          = BIT!(1),
    EventKeyboard       = BIT!(2),
    EventMouse          = BIT!(3),
    EventMouseButton    = BIT!(4)
}

pub trait Event {

    fn get_event_type(&self) -> EventType;

    fn get_category_flags(&self) -> u32;

    fn get_msg(&self) -> &String;

    fn get_data(&self) -> Option<& Vec<f32>>;

    fn handled(&mut self) -> &mut bool;
}

pub struct EventDispatcher<'a> {
    event_type: EventType,
    event: &'a mut (dyn Event + 'a)
}

impl<'a> EventDispatcher<'a> {

    pub fn new(ev_type: EventType, _event: &'a mut (dyn Event + 'a)) -> EventDispatcher<'a> {
        EventDispatcher { event_type: ev_type, event: _event }
    }

    pub fn dispatch<T>(&mut self, func: fn(&mut (dyn Event + 'a)) -> bool) -> bool {
        if self.event_type == self.event.get_event_type() {
            let x = func(self.event);
            return x;
        }
        false
    }
}
