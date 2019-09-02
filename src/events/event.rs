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

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::None => write!(f, "(None)", ),
            EventType::WindowClose => write!(f, "WindowClose"),
            EventType::WindowResize => write!(f, "WindowResize"),
            EventType::WindowFocus => write!(f, "WindowFocus"),
            EventType::WindowLostFocus => write!(f, "WindowLostFocus"),
            EventType::WindowMoved => write!(f, "WindowMoved"),
            EventType::AppTick => write!(f, "AppTick"),
            EventType::AppUpdate => write!(f, "AppUpdate"),
            EventType::AppRender => write!(f, "AppRender"),
            EventType::KeyPressed => write!(f, "KeyPressed"),
            EventType::KeyReleased => write!(f, "KeyReleased"),
            EventType::MouseButtonPressed => write!(f, "MouseButtonPressed"),
            EventType::MouseButtonReleased => write!(f, "MouseButtonReleased"),
            EventType::MouseMoved => write!(f, "MouseMoved"),
            EventType::MouseScrolled => write!(f, "MouseScrolled")
        }
    }
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

pub trait Event : std::fmt::Display {

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

pub type EventCallbackFn = fn(&mut (dyn Event))-> bool;

impl<'a> EventDispatcher<'a> {

    pub fn new(ev_type: EventType, event: &'a mut (dyn Event + 'a)) -> EventDispatcher<'a> {
        EventDispatcher { event_type: ev_type, event: event }
    }

    pub fn dispatch<T>(&mut self, func: EventCallbackFn) -> bool {
        if self.event_type == self.event.get_event_type() {
            let x = func(self.event);
            return x;
        }
        false
    }
}
