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

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Event {
    event_type: EventType,
    static_event_type: EventType,
    category_flags: u32, //could be smaller, say u8, but will keep u32 for future extensibility
    name: String,
    data: Vec<f32>,
    handled: bool
}

impl Event {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event { event_type: event_type,
                static_event_type: static_event_type,
                category_flags: category_flags,
                name: name,
                data: data,
                handled: handled 
              }
    }

    #[cfg(debug_assertions)]
    pub fn to_string(self) -> String {
        self.name
    }

    #[inline(always)]
    pub fn is_in_category(&self, category: EventCategory) -> bool {
        if self.category_flags & category as u32 > 0 { true } else { false }
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    pub fn handled(&self) -> bool { self.handled }
}

pub struct EventDispatcher {
    event: Event
}

impl EventDispatcher {

    pub fn new(event: Event) -> EventDispatcher {
        EventDispatcher { event: event }
    }

    pub fn dispatch<T>(&mut self, func: fn(&Event) -> bool) -> bool {
        if self.event.static_event_type == self.event.event_type {
            let x = func(&self.event);
            return x;
        }
        false
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowResizeEvent {
    event: Event
}

impl WindowResizeEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowClosedEvent {
    event: Event
}

impl WindowClosedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }
    
    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowFocusEvent {
    event: Event
}

impl WindowFocusEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }
    
    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowMovedEvent {
    event: Event
}

impl WindowMovedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppTickEvent {
    event: Event
}

impl AppTickEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppUpdateEvent {
    event: Event
}

impl AppUpdateEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct AppRenderEvent {
    event: Event
}

impl AppRenderEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct KeyPressedEvent {
    event: Event
}

impl KeyPressedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct KeyReleasedEvent {
    event: Event
}

impl KeyReleasedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseMovedEvent {
    event: Event
}

impl MouseMovedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseScrolledEvent {
    event: Event
}

impl MouseScrolledEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseButtonPressedEvent {
    event: Event
}

impl MouseButtonPressedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MouseButtonReleasedEvent {
    event: Event
}

impl MouseButtonReleasedEvent {
    pub fn new(event_type: EventType, static_event_type: EventType, category_flags: u32, name: String, data: Vec<f32>, handled: bool) -> Event {
        Event::new(event_type, static_event_type, category_flags, name, data, handled)
    }

    pub fn set_handled(&mut self, handled: bool) {
        self.event.set_handled(handled);
    }

    pub fn handled(&self) -> bool { self.event.handled() }
}
