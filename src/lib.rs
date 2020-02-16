#[macro_use]
extern crate log;

extern crate fern;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate sigs_slots;

extern crate libc;
extern crate gl;
extern crate glfw;
extern crate raw_window_handle;
extern crate vulkano;
extern crate vulkano_glfw_v2 as vulkano_glfw;

#[cfg(windows)]
extern crate dxplr;

#[macro_use]
pub mod core;
pub mod events;
