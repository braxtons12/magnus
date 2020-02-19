use std::sync::mpsc::Receiver;
use std::sync::{ Arc, RwLock };

use glfw;

use crate::core::signals::{ SyncData, SyncSignal, SyncSlot, SyncSlotPair };
use crate::core::settings::GraphicsMode;
use crate::events::event::{ EventType, EventData, Event };
use crate::events::key_events::*;
use crate::events::mouse_events::*;
use crate::events::window_events::*;
use crate::core::graphics;
use crate::core::graphics::context::ContextLimiter;
use crate::core::graphics::opengl::OpenGLContext;
use crate::core::graphics::vulkan::VulkanContext;
#[cfg(windows)]
use crate::core::graphics::directx::DirectXContext;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
    graphics_mode: GraphicsMode
}

impl WindowProps {
    pub fn new(title: String, size: Option<(u32, u32)>, graphics_mode: GraphicsMode) -> WindowProps {
        WindowProps {
            title,
            width: match size {
                Some((w,_h)) => w,
                None => 800
            },
            height: match size {
                Some((_w, h)) => h,
                None => 600
            },
            graphics_mode
        }
    }
}

#[repr(C)]
pub struct Window<T: ContextLimiter> {
    props: WindowProps,
    vsync: u8,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    context: graphics::context::Context<T>,
    slots: Vec<SyncSlotPair>,
    should_close: bool
}

unsafe impl<T: ContextLimiter> std::marker::Send for Window<T> {}
unsafe impl<T: ContextLimiter> std::marker::Sync for Window<T> {}

static mut GLFW_S: Option<glfw::Glfw> = None;

impl Window<OpenGLContext> {
    pub fn new(props: WindowProps) -> Window<OpenGLContext> {
        use glfw::Context;

        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
            }
        }

        debug!("Creating Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        window.make_current();

        let context = graphics::context::Context::<OpenGLContext>::new(window);

        Window {
            props,
            vsync: 0,
            event_receiver: events,
            context,
            slots: vec![],
            should_close: false
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<OpenGLContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();

        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let x = WindowCloseEvent::new("Window Should Close".to_string());
                        let res = self.emit(SyncData::Sig(x));
                        debug!("Emit Result: {:?}", res);
                        return true;
                    }
                }
            }

        }
        self.should_close
    }
}

impl Window<VulkanContext> {
    pub fn new(props: WindowProps, id: usize) -> Window<VulkanContext> {
        use vulkano::instance::Instance;

        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
            }
        }

        debug!("Creating Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatibility");
            GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);
        let ext = &vulkano_glfw::get_required_instance_extensions(&window.glfw)
            .expect("Error getting required vulkan instance extensions");
        let instance = Instance::new(None, ext, None).expect("Failed to create vulkan instance");

        Window {
            props,
            vsync: 0,
            event_receiver: events,
            context: graphics::context::Context::<VulkanContext>::new(window, instance, id),
            slots: vec![],
            should_close: false
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<VulkanContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();

        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let x = WindowCloseEvent::new("Window Should Close".to_string());
                        let res = self.emit(SyncData::Sig(x));
                        debug!("Emit Result: {:?}", res);
                        self.should_close = true;
                    }
                }
            }

        }
        self.should_close
    }


}

#[cfg(windows)]
impl Window<DirectXContext> {
    pub fn new(props: WindowProps) -> Window<DirectXContext> {

        unsafe {
            if GLFW_S.is_none() {
                GLFW_S = Some(glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW"));
            }
        }

        debug!("Creating Window: {}", props.title);
        let mut window: glfw::Window;
        let events: Receiver<(f64, glfw::WindowEvent)>;
        let x: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>);
        unsafe {
            debug!("Setting ClientAPI WindowHint to NoApi for vulkan/directx compatibility");
            GLFW_S.unwrap().window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
            debug!("Creating glfw window");
            x = GLFW_S.unwrap().create_window(props.width, props.height, props.title.as_str(),
            glfw::WindowMode::Windowed).expect("Failed to create GLFW Window");
        }

        window = x.0;
        events = x.1;
        window.set_all_polling(true);

        let context = graphics::context::Context::<DirectXContext>::new(window);

        Window {
            props,
            vsync: 0,
            event_receiver: events,
            context,
            slots: vec![],
            should_close: false
        }
    }

    pub fn get_context(&mut self) -> &mut graphics::context::Context<DirectXContext> {
        &mut self.context
    }

    pub fn get_width(&self) -> u32 {
        self.props.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.props.width = w;
        self.context.set_width(w);
    }

    pub fn get_height(&self) -> u32 {
        self.props.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.props.height = h;
        self.context.set_height(h);
    }

    pub fn get_vsync(&self) -> u8 {
        self.vsync
    }

    pub fn set_vsync(&mut self, interval: u8) {
        self.context.set_vsync(interval);
        match interval {
            0..=2 => {
                self.vsync = interval;
            },
            _ => {
                self.vsync = 0;
            }
        }
    }

    pub fn get_props(&self) -> & WindowProps {
        &self.props
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn on_update(&mut self) -> bool {
        self.context.poll_events();

        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(_, id, glfw::Action::Press, mods) => {
                    let x = KeyPressedEvent::new(format!("Key {} pressed with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Key(_, id, glfw::Action::Release, mods) => {
                    let x = KeyReleasedEvent::new(format!("Key {} released with {} mods", id, mods.bits()), id, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Press, mods) => {
                    let x = MouseButtonPressedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::MouseButton(button, glfw::Action::Release, mods) => {
                    let x = MouseButtonReleasedEvent::new(format!("Mouse Button {} pressed with {} mods", button as i32, mods.bits()), button as i32, mods.bits());
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);                                                ;
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    let x = MouseScrolledEvent::new(format!("Mouse Scrolled x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = MouseMovedEvent::new(format!("Mouse Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Focus(focus) => {
                    let x = WindowFocusEvent::new("Window Focused".to_string(), focus);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Pos(x, y) => {
                    let x = WindowMovedEvent::new(format!("Window Moved x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                glfw::WindowEvent::Size(x, y) => {
                    let x = WindowResizeEvent::new(format!("Window Resized x: {}, y: {}", x, y), x as f32, y as f32);
                    let res = self.emit(SyncData::Sig(x));
                    debug!("Emit Result: {:?}", res);
                },
                _ => {
                    if event == glfw::WindowEvent::Close {
                        let x = WindowCloseEvent::new("Window Should Close".to_string());
                        let res = self.emit(SyncData::Sig(x));
                        debug!("Emit Result: {:?}", res);
                        self.should_close = true;
                    }
                }
            }
        }
        self.should_close
    }
}

impl<T: ContextLimiter> SyncSignal<WindowCloseEvent, EventData> for Window<T> {
    fn connect<WindowCloseEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowClose ));
    }

    fn emit(&self, _event: SyncData<WindowCloseEvent>) -> Result<(), &str> {
        let temp_event = WindowCloseEvent::new("Window Closing".to_string());
        let temp_data = match temp_event.get_data() {
            Some(x) => x,
            None => return Err("Failed to unwrap temporary WindowCloseEvent data")
        };
        let data = SyncData::Sig(temp_data);

        let mut handled = false;
        for slot in &self.slots {
            if slot.1 == EventType::WindowClose && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowResizeEvent, EventData> for Window<T> {
    fn connect<WindowResizeEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowResize));
    }

    fn emit(&self, event: SyncData<WindowResizeEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowResize && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowFocusEvent, EventData> for Window<T> {
    fn connect<WindowFocusEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowFocus));
    }

    fn emit(&self, event: SyncData<WindowFocusEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowFocus && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowMovedEvent, EventData> for Window<T> {
    fn connect<WindowMovedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowMoved));
    }

    fn emit(&self, event: SyncData<WindowMovedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowRefreshEvent, EventData> for Window<T> {
    fn connect<WindowRefreshEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowRefresh));
    }

    fn emit(&self, event: SyncData<WindowRefreshEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowIconifyEvent, EventData> for Window<T> {
    fn connect<WindowIconifyEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowIconify));
    }

    fn emit(&self, event: SyncData<WindowIconifyEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data(){
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<WindowMaximizeEvent, EventData> for Window<T> {
    fn connect<WindowMaximizeEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::WindowMaximize));
    }

    fn emit(&self, event: SyncData<WindowMaximizeEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<KeyPressedEvent, EventData> for Window<T> {
    fn connect<KeyPressedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::KeyPressed));
    }

    fn emit(&self, event: SyncData<KeyPressedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<KeyReleasedEvent, EventData> for Window<T> {
    fn connect<KeyReleasedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::KeyReleased));
    }

    fn emit(&self, event: SyncData<KeyReleasedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<MouseButtonPressedEvent, EventData> for Window<T> {
    fn connect<MouseButtonPressedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::MouseButtonPressed));
    }

    fn emit(&self, event: SyncData<MouseButtonPressedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<MouseButtonReleasedEvent, EventData> for Window<T> {
    fn connect<MouseButtonReleasedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::MouseButtonReleased));
    }

    fn emit(&self, event: SyncData<MouseButtonReleasedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<MouseEnteredEvent, EventData> for Window<T> {
    fn connect<MouseEnteredEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::MouseEntered));
    }

    fn emit(&self, event: SyncData<MouseEnteredEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<MouseMovedEvent, EventData> for Window<T> {
    fn connect<MouseMovedEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::MouseMoved));
    }

    fn emit(&self, event: SyncData<MouseMovedEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

impl<T: ContextLimiter> SyncSignal<MouseScrolledEvent, EventData> for Window<T> {
    fn connect<MouseScrolledEvent>(&mut self, slot: Arc<RwLock<dyn SyncSlot<EventData>>>) {
        self.slots.push((slot, EventType::MouseScrolled));
    }

    fn emit(&self, event: SyncData<MouseScrolledEvent>) -> Result<(), &str> {
        let ev = event.sig();
        let data = SyncData::Sig(match ev.get_data() {
            Some(x) => x,
            None => return Err("No data in event!")
        });
        let mut handled = ev.get_handled();
        for slot in &self.slots {
            if slot.1 == EventType::WindowMoved && !handled {
                handled = match slot.0.write() {
                    Ok(mut x) => x.consume(&data),
                    _ => {
                        debug!("Unable to lock slot for signal consumption");
                        false
                    }
                }
            }
        }
        Ok(())
    }
}

