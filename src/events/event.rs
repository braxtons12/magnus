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

    fn get_handled(&self) -> bool;

    fn set_handled(&mut self, handled: bool);
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
    Bool(bool, EventType),
    Boolp(bool, bool, EventType),
    Bools(Vec<bool>, EventType),
    U32(u32, EventType),
    U32p(u32, u32, EventType),
    U32s(Vec<u32>, EventType),
    U64(u64, EventType),
    U64p(u64, u64, EventType),
    U64s(Vec<u64>, EventType),
    I32(i32, EventType),
    I32p(i32, i32, EventType),
    I32s(Vec<i32>, EventType),
    I64(i64, EventType),
    I64p(i64, i64, EventType),
    I64s(Vec<i64>, EventType),
    F32(f32, EventType),
    F32p(f32, f32, EventType),
    F32s(Vec<f32>, EventType),
    F64(f64, EventType),
    F64p(f64, f64, EventType),
    F64s(Vec<f64>, EventType),
    StringD(String, EventType),
    Strings(Vec<String>, EventType),
    PathBufD(std::path::PathBuf, EventType),
    PathBufs(Vec<std::path::PathBuf>, EventType),
}

impl EventData {
    pub fn event_type(&self) -> &EventType {
        match self {
            EventData::Bool(_, kind) => kind,
            EventData::Boolp(_, _, kind) => kind,
            EventData::Bools(_, kind) => kind,
            EventData::U32(_, kind) => kind,
            EventData::U32p(_, _, kind) => kind,
            EventData::U32s(_, kind) => kind,
            EventData::U64(_, kind) => kind,
            EventData::U64p(_, _, kind) => kind,
            EventData::U64s(_, kind) => kind,
            EventData::I32(_, kind) => kind,
            EventData::I32p(_, _, kind) => kind,
            EventData::I32s(_, kind) => kind,
            EventData::I64(_, kind) => kind,
            EventData::I64p(_, _, kind) => kind,
            EventData::I64s(_, kind) => kind,
            EventData::F32(_, kind) => kind,
            EventData::F32p(_, _, kind) => kind,
            EventData::F32s(_, kind) => kind,
            EventData::F64(_, kind) => kind,
            EventData::F64p(_, _, kind) => kind,
            EventData::F64s(_, kind) => kind,
            EventData::StringD(_, kind) => kind,
            EventData::Strings(_, kind) => kind,
            EventData::PathBufD(_, kind) => kind,
            EventData::PathBufs(_, kind) => kind
        }
    }
}

unsafe impl std::marker::Send for EventData {}

impl std::fmt::Display for EventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventData::Bool(x, kind) => write!(f, "EventData::Bool: {}, type: {}", x, kind),
            EventData::Boolp(x, y, kind) => write!(f, "EventData::Boolp: {}, {}, type: {}", x, y, kind),
            EventData::Bools(x, kind) => write!(f, "EventData::Bools: size: {}, type: {}", x.len(), kind),
            EventData::U32(x, kind) => write!(f, "EventData::U32: {}, type: {}", x, kind),
            EventData::U32p(x, y, kind) => write!(f, "EventData::U32p: {}, {}, type: {}", x, y, kind),
            EventData::U32s(x, kind) => write!(f, "EventData::U32s: size: {}, type: {}", x.len(), kind),
            EventData::U64(x, kind) => write!(f, "EventData::U64: {}, type: {}", x, kind),
            EventData::U64p(x, y, kind) => write!(f, "EventData::U64p: {}, {}, type: {}", x, y, kind),
            EventData::U64s(x, kind) => write!(f, "EventData::U64s: size: {}, type: {}", x.len(), kind),
            EventData::I32(x, kind) => write!(f, "EventData::I32: {}, type: {}", x, kind),
            EventData::I32p(x, y, kind) => write!(f, "EventData::I32p: {}, {}, type: {}", x, y, kind),
            EventData::I32s(x, kind) => write!(f, "EventData::I32s: size: {}, type: {}", x.len(), kind),
            EventData::I64(x, kind) => write!(f, "EventData::I64: {}, type: {}", x, kind),
            EventData::I64p(x, y, kind) => write!(f, "EventData::I64p: {}, {}, type: {}", x, y, kind),
            EventData::I64s(x, kind) => write!(f, "EventData::I64s: size: {}, type: {}", x.len(), kind),
            EventData::F32(x, kind) => write!(f, "EventData::F32: {}, type: {}", x, kind),
            EventData::F32p(x, y, kind) => write!(f, "EventData::F32p: {}, {}, type: {}", x, y, kind),
            EventData::F32s(x, kind) => write!(f, "EventData::F32s: size: {}, type: {}", x.len(), kind),
            EventData::F64(x, kind) => write!(f, "EventData::F64: {}, type: {}", x, kind),
            EventData::F64p(x, y, kind) => write!(f, "EventData::F64p: {}, {}, type: {}", x, y, kind),
            EventData::F64s(x, kind) => write!(f, "EventData::F64s: size: {}, type: {}", x.len(), kind),
            EventData::StringD(x, kind) => write!(f, "EventData::StringD: {}, type: {}", x, kind),
            EventData::Strings(x, kind) => write!(f, "EventData::Strings: size: {}, type: {}", x.len(), kind),
            EventData::PathBufD(x, kind) => write!(f, "EventData::PathBufD: to_str: {}, type: {}", x.to_str().unwrap(), kind),
            EventData::PathBufs(x, kind) => write!(f, "EventData::PathBufs: size: {}, type: {}", x.len(), kind)
        }
    }
}
