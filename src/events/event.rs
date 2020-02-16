#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum EventType {
    None = 0,
    WindowClose, WindowResize, WindowFocus, WindowMoved,
    WindowRefresh, WindowIconify, WindowMaximize,
    RenderFramebufferResize, RenderContentScaleResize,
    AppTick, AppUpdate, AppRender, AppFileDropped,
    KeyPressed, KeyReleased, TextInput,
    MouseButtonPressed, MouseButtonReleased, MouseEntered, MouseMoved, MouseScrolled
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::None => write!(f, "(None)", ),
            EventType::WindowClose => write!(f, "WindowClose"),
            EventType::WindowResize => write!(f, "WindowResize"),
            EventType::WindowFocus => write!(f, "WindowFocus"),
            EventType::WindowMoved => write!(f, "WindowMoved"),
            EventType::WindowRefresh => write!(f, "WindowRefresh"),
            EventType::WindowIconify => write!(f, "WindowIconify"),
            EventType::WindowMaximize => write!(f, "WindowMaximize"),
            EventType::RenderFramebufferResize => write!(f, "RenderFramebufferResize"),
            EventType::RenderContentScaleResize => write!(f, "RenderContentScaleResize"),
            EventType::AppTick => write!(f, "AppTick"),
            EventType::AppUpdate => write!(f, "AppUpdate"),
            EventType::AppRender => write!(f, "AppRender"),
            EventType::AppFileDropped => write!(f, "AppFileDropped"),
            EventType::KeyPressed => write!(f, "KeyPressed"),
            EventType::KeyReleased => write!(f, "KeyReleased"),
            EventType::TextInput => write!(f, "TextChar"),
            EventType::MouseButtonPressed => write!(f, "MouseButtonPressed"),
            EventType::MouseButtonReleased => write!(f, "MouseButtonReleased"),
            EventType::MouseEntered => write!(f, "MouseEntered"),
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

    fn get_data(&self) -> Option<& EventData>;

    fn handled(&mut self) -> &mut bool;
}

pub struct EventDispatcher<'a> {
    event_type: EventType,
    event: &'a mut (dyn Event + 'a)
}

pub type EventCallbackFn = fn(&mut (dyn Event))-> bool;

impl<'a> EventDispatcher<'a> {

    pub fn new(ev_type: EventType, event: &'a mut (dyn Event + 'a)) -> EventDispatcher<'a> {
        EventDispatcher { event_type: ev_type, event }
    }

    pub fn dispatch<T>(&mut self, func: EventCallbackFn) -> bool {
        if self.event_type == self.event.get_event_type() {
            let x = func(self.event);
            return x;
        }
        false
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EventData {
    Bool(bool),
    Boolp(bool, bool),
    Bools(Vec<bool>),
    U32(u32),
    U32p(u32, u32),
    U32s(Vec<u32>),
    U64(u64),
    U64p(u64, u64),
    U64s(Vec<u64>),
    I32(i32),
    I32p(i32, i32),
    I32s(Vec<i32>),
    I64(i64),
    I64p(i64, i64),
    I64s(Vec<i64>),
    F32(f32),
    F32p(f32, f32),
    F32s(Vec<f32>),
    F64(f64),
    F64p(f64, f64),
    F64s(Vec<f64>),
    StringD(String),
    Strings(Vec<String>),
    PathBufD(std::path::PathBuf),
    PathBufs(Vec<std::path::PathBuf>),
}

impl std::fmt::Display for EventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventData::Bool(x) => write!(f, "EventData::Bool: {}", x),
            EventData::Boolp(x, y) => write!(f, "EventData::Boolp: {}, {}", x, y),
            EventData::Bools(x) => write!(f, "EventData::Bools: size: {}", x.len()),
            EventData::U32(x) => write!(f, "EventData::U32: {}", x),
            EventData::U32p(x, y) => write!(f, "EventData::U32p: {}, {}", x, y),
            EventData::U32s(x) => write!(f, "EventData::U32s: size: {}", x.len()),
            EventData::U64(x) => write!(f, "EventData::U64: {}", x),
            EventData::U64p(x, y) => write!(f, "EventData::U64p: {}, {}", x, y),
            EventData::U64s(x) => write!(f, "EventData::U64s: size: {}", x.len()),
            EventData::I32(x) => write!(f, "EventData::I32: {}", x),
            EventData::I32p(x, y) => write!(f, "EventData::I32p: {}, {}", x, y),
            EventData::I32s(x) => write!(f, "EventData::I32s: size: {}", x.len()),
            EventData::I64(x) => write!(f, "EventData::I64: {}", x),
            EventData::I64p(x, y) => write!(f, "EventData::I64p: {}, {}", x, y),
            EventData::I64s(x) => write!(f, "EventData::I64s: size: {}", x.len()),
            EventData::F32(x) => write!(f, "EventData::F32: {}", x),
            EventData::F32p(x, y) => write!(f, "EventData::F32p: {}, {}", x, y),
            EventData::F32s(x) => write!(f, "EventData::F32s: size: {}", x.len()),
            EventData::F64(x) => write!(f, "EventData::F64: {}", x),
            EventData::F64p(x, y) => write!(f, "EventData::F64p: {}, {}", x, y),
            EventData::F64s(x) => write!(f, "EventData::F64s: size: {}", x.len()),
            EventData::StringD(x) => write!(f, "EventData::StringD: {}", x),
            EventData::Strings(x) => write!(f, "EventData::Strings: size: {}", x.len()),
            EventData::PathBufD(x) => write!(f, "EventData::PathBufD: to_str: {}", x.to_str().unwrap()),
            EventData::PathBufs(x) => write!(f, "EventData::PathBufs: size: {}", x.len())
        }
    }
}
