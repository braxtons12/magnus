pub mod opengl;
pub mod directx;

use std::error::Error;
use std::fmt;

pub struct Context<'a> {
    context_wrapper: &'a mut (dyn ContextWrapper + 'a)
}

impl<'a> Context<'a> {
    pub fn new(wrapper: &'a mut (dyn ContextWrapper + 'a)) -> Context {
        Context { context_wrapper: wrapper }
    }

    pub fn load_symbols(&mut self) {
        debug!("Context loading symbols");
        self.context_wrapper.load_symbols();
    }
}

pub trait ContextWrapper {
    fn load_symbols(&mut self);
}

impl ContextWrapper for &mut dyn ContextWrapper {
    fn load_symbols(&mut self) {
        self.load_symbols();
    }
}

#[derive(Debug)]
struct SymbolLoadError {
    details: String
}

impl SymbolLoadError {
    fn new(msg: String) -> SymbolLoadError {
        SymbolLoadError { details: msg }
    }
}

impl fmt::Display for SymbolLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SymbolLoadError {
    fn description(&self) -> & str {
        & self.details
    }
}
