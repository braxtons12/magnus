/**User MUST declare #![no_main] in their main.rs
 *
 * User MUST implement this function for a struct serving as their app
 * #[no_mangle]
 * pub fn create_application() -> impl Box<Application>;
 *
 **/
use std::any::Any;
use std::sync::{ Arc, RwLock };
use crate::core::signals::{ SyncData, SyncSignal, SyncSlot, SyncSlotPair };
use crate::events::event::*;
use crate::events::event::Event;
use crate::events::window_events::*;
use crate::events::mouse_events::*;
use crate::events::key_events::*;
use crate::core::settings::Settings;
use crate::core::settings::GraphicsMode;
use crate::core::graphics;
use crate::core::graphics::opengl::OpenGLContext;
use crate::core::graphics::vulkan::VulkanContext;
#[cfg(windows)]
use crate::core::graphics::directx::DirectXContext;
use crate::core::window::*;
use crate::core::layers::*;

#[repr(C)]
pub struct MagnusApplication<T: graphics::context::ContextLimiter> {
    name: String,
    running: bool,
    settings: Settings,
    window: Window<T>,
    layer_stack: LayerStack,
    event_handler: Arc<RwLock<dyn SyncSlot<EventData>>>,
}

impl MagnusApplication<OpenGLContext> {
    pub fn new(name: String, settings: Settings) -> MagnusApplication<OpenGLContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<OpenGLContext>::new(props),
            layer_stack: LayerStack::new(None, None),
            event_handler: EventHandler::new(),
        }
    }

    pub fn run(mut self)  {
        use std::thread;
        use std::sync::atomic::{ AtomicBool, Ordering };

        debug!("Application {} Started", self.name);
        self.window.get_context().api_context().load_symbols().expect("Failed to load graphics context symbols");
        self.window.set_vsync(0);
        SyncSignal::<WindowResizeEvent, EventData>::connect::<WindowResizeEvent>(&mut self.window, Arc::clone(&self.event_handler));
        SyncSignal::<WindowCloseEvent, EventData>::connect::<WindowCloseEvent>(&mut self.window, Arc::clone(&self.event_handler));
        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
        }
        let close_backup = Arc::new(AtomicBool::new(false));
        let event_handler = Arc::clone(&self.event_handler);
        let window = Arc::new(RwLock::new(self.window));
        let layer_stack = Arc::new(RwLock::new(self.layer_stack));
        debug!("Starting update thread");
        let update_thread = {
            let closed_backup = Arc::clone(&close_backup);
            let stack = Arc::clone(&layer_stack);
            let win = Arc::clone(&window);
            thread::spawn(move || {
                let mut close = false;
                'update: while !close {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    match win.write() {
                        Ok(mut x) => if x.on_update() {
                            warn!("App should close!");
                            close = true;
                        },
                        _ => {
                            error!("Window RWLock is Poisoned (Update Thread)");
                            error!("Signalling to shutdown");
                            close = true;
                        }
                    }

                    if close {
                        //shutting down this thread can poison the shared RWLocks
                        //so need a backup way to tell main to shutdown other than just
                        //the signal emitted to the event handler
                        warn!("Update thread shutting down");
                        closed_backup.store(true, Ordering::SeqCst);
                        break 'update;
                    }

                    match stack.try_write() {
                        Ok(mut x) => for item in x.iter_mut() {
                            item.on_update();
                        },
                        _ => debug!("Unable to lock layer stack for updates")
                    }
                }
            })
        };

        let mut close = false;
        let mut frames: usize = 0;
        let mut timer_start = std::time::Instant::now();
        'main: while !close {

            if close_backup.load(Ordering::SeqCst) {
                warn!("Main/Render thread shutting down via backup signal");
                break 'main;
            }

            match event_handler.try_read() {
                Ok(x) => if x.as_any().downcast_ref::<EventHandler>().unwrap().should_close() {
                    close = true;
                }
                _ => debug!("Unable to lock event_handler for should_close check")
            }

            if close {
                warn!("Main/Render thread shutting down normally");
                break 'main;
            }

            //debug!("Window width is {}", self.window.get_width());
            //debug!("Changing clear color");
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            match window.write() {
                Ok(mut x) => x.get_context().swap_buffers(),
                _ => {
                    error!("Window RWLock is Poisoned (Render Thread)");
                    error!("Assuming close signal has been given");
                    error!("Shutting down");
                    return;
                }
            }
            frames += 1;
            if std::time::Instant::now().duration_since(timer_start).as_millis() > 1000 {
                debug!("{} FPS", frames);
                frames = 0;
                timer_start = std::time::Instant::now();
            }
        }
        update_thread.join();
    }
}

impl MagnusApplication<VulkanContext> {
    pub fn new(name: String, settings: Settings) -> MagnusApplication<VulkanContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<VulkanContext>::new(props, settings.graphics().vulkan_id()),
            layer_stack: LayerStack::new(None, None),
            event_handler: EventHandler::new(),
        }
    }

    pub fn run(mut self)  {
        use std::thread;

        debug!("Application {} Started", self.name);
        debug!("Starting update thread");
        let settings = Arc::new(RwLock::new(self.settings));
        let window = Arc::new(RwLock::new(self.window));
        let layer_stack = Arc::new(RwLock::new(self.layer_stack));
        let update_thread = {
            let stack = Arc::clone(&layer_stack);
            let win = Arc::clone(&window);
            thread::spawn(move || {
                loop {
                    match stack.try_write() {
                        Ok(mut x) => for item in x.iter_mut() {
                            item.on_update();
                        },
                        _ => debug!("Unable to lock layer stack for updates")
                    }

                    match win.try_write() {
                        Ok(mut x) => if x.on_update() {
                            return
                        },
                        _ => debug!("Unable to lock window for updates")
                    }
                }
            })
        };
        'main: loop {
            debug!("Changing clear color");
            unsafe {
                match settings.try_read() {
                    Ok(x) => if x.graphics().mode() == GraphicsMode::OpenGL {
                        gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    },
                    _ => debug!("Unable to lock settings for graphics mode check")
                }
            }

            match window.try_read() {
                Ok(x) => if x.should_close() {
                    break 'main;
                },
                _ => debug!("Unable to lock window for should_close check")
            }
        }
        update_thread.join();
    }
}

#[cfg(windows)]
impl MagnusApplication<DirectXContext> {
    pub fn new(name: String, width: i32, height: i32, settings: Settings) -> MagnusApplication<DirectXContext> {
        let props = WindowProps::new(name.clone(), Some(settings.graphics().size()), settings.graphics().mode());

        MagnusApplication {
            name,
            running: true,
            settings,
            window: Window::<DirectXContext>::new(props),
            layer_stack: LayerStack::new(None, None),
            event_handler: EventHandler::new()
        }
    }

    pub fn run(mut self)  {

        debug!("Application {} Started", self.name);
        'main: loop {
            debug!("Window width is {}", self.window.get_width());
            debug!("Changing clear color");
            unsafe {
                if self.settings.graphics().mode() == GraphicsMode::OpenGL {
                    gl::ClearColor(1.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
            for item in self.layer_stack.iter_mut() {
                item.on_update();
            }
            if self.window.on_update() {
                break 'main;
            }
        }
    }
}

impl<T: graphics::context::ContextLimiter> MagnusApplication<T> {
    pub fn push_layer(&mut self, layer: Layer) {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, layer: Layer) {
        self.layer_stack.push_overlay(layer);
    }

    pub fn remove_layer(&mut self) -> Layer {
        self.layer_stack.remove_layer()
    }

    pub fn remove_overlay(&mut self) -> Layer {
        self.layer_stack.remove_overlay()
    }

    pub fn layer_stack(&mut self) -> &mut LayerStack {
        &mut self.layer_stack
    }

    #[inline(always)]
    pub fn get_running(&self) -> bool {
        self.running
    }

    #[inline(always)]
    pub fn set_running(&mut self, set: bool) {
        self.running = set;
    }

    //#[inline(always)]
    //fn get_layer_stack(&self) -> LayerStack

    pub fn on_window_resize(&mut self, e: &mut dyn Event) -> bool {
        e.set_handled(false);
        false
    }

    pub fn on_window_close(&mut self, e: &mut dyn Event) -> bool {
        e.set_handled(true);
        self.set_running(false);
        true
    }

    //pub fn on_event(e: &mut (dyn Event)) -> bool {

    //debug!("Processing event e: {}", e);
    //unsafe {
    //let app = APP.as_mut().unwrap();
    //'event: for item in app.layer_stack().iter_mut() {
    //item.on_event(e);
    //if * e.handled() {
    //break 'event
    //}
    //}
    //}
    //true
    //}
}

struct EventHandler {
    slots: Vec<SyncSlotPair>,
    should_close: bool
}

impl EventHandler {
    pub fn new() -> Arc<RwLock<dyn SyncSlot<EventData>>> {
        Arc::new(RwLock::new(EventHandler{ slots: vec![], should_close: false }))
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }
}

unsafe impl std::marker::Send for EventHandler {}

impl SyncSlot<EventData> for EventHandler {
    #[allow(clippy::cognitive_complexity)]
    fn consume(&mut self, event: &SyncData<&EventData>) -> bool {
        let data = *event.sig();
        match data.event_type() {
            EventType::None => {},
            EventType::RenderFramebufferResize => {},
            EventType::RenderContentScaleResize => {},
            EventType::AppTick => {},
            EventType::AppUpdate => {},
            EventType::AppRender => {},
            EventType::AppFileDropped => {},
            EventType::TextInput => {},
            EventType::KeyPressed => {
                if let EventData::I32p(key, mods, _) = data {
                    debug!("Key {} pressed with mods: {}", key, mods);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::KeyReleased => {
                if let EventData::I32p(key, mods, _) = data {
                    debug!("Key {} released with mods: {}", key, mods);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::MouseMoved => {
                if let EventData::F32p(x, y, _) = data {
                    debug!("Mouse moved: x: {}, y: {}", x, y);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::MouseEntered => {
                debug!("Mouse entered");
            },
            EventType::MouseScrolled => {
                if let EventData::F32p(x, y, _) = data {
                    debug!("Mouse scrolled: x: {}, y: {}", x, y);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::MouseButtonPressed => {
                if let EventData::I32p(button, mods, _) = data {
                    debug!("Button {} pressed with mods: {}", button, mods);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::MouseButtonReleased => {
                if let EventData::I32p(button, mods, _) = data {
                    debug!("Button {} pressed with mods: {}", button, mods);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::WindowClose => {
                self.should_close = true;
                debug!("Window closing");
            },
            EventType::WindowFocus => {
                if let EventData::Bool(focused, _) = data {
                    debug!("Window Focused: {}", focused);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::WindowMoved => {
                if let EventData::F32p(x, y, _) = data {
                    debug!("Window moved: x: {}, y: {}", x, y);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::WindowResize => {
                if let EventData::F32p(x, y, _) = data {
                    debug!("Window resized: x: {}, y: {}", x, y);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::WindowRefresh => {
                debug!("Window refreshed");
            },
            EventType::WindowIconify => {
                if let EventData::Bool(iconified, _) = data {
                    debug!("Window Iconified: {}", iconified);
                }
                else {
                    panic!("This will never happen");
                }
            },
            EventType::WindowMaximize => {
                if let EventData::Bool(maximized, _) = data {
                    debug!("Window maximized: {}", maximized);
                }
                else {
                    panic!("This will never happen");
                }
            },
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

