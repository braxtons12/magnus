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
            WindowClose => write!(f, "WindowClose"),
            WindowResize => write!(f, "WindowResize"),
            WindowFocus => write!(f, "WindowFocus"),
            WindowLostFocus => write!(f, "WindowLostFocus"),
            WindowMoved => write!(f, "WindowMoved"),
            AppTick => write!(f, "AppTick"),
            AppUpdate => write!(f, "AppUpdate"),
            AppRender => write!(f, "AppRender"),
            KeyPressed => write!(f, "KeyPressed"),
            KeyReleased => write!(f, "KeyReleased"),
            MouseButtonPressed => write!(f, "MouseButtonPressed"),
            MouseButtonReleased => write!(f, "MouseButtonReleased"),
            MouseMoved => write!(f, "MouseMoved"),
            MouseScrolled => write!(f, "MouseScrolled")
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
