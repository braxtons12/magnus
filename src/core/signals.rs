use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;

use crate::events::event::{ EventData, EventType };

pub type SyncSlotPair = (Arc<RwLock<dyn SyncSlot<EventData>>>, EventType);
pub type SlotPair = (Rc<RefCell<dyn Slot<EventData>>>, EventType);

pub enum Data<T> {
    Sig(T),
}

impl<T> Data<T> {
    #[allow(irrefutable_let_patterns)]
    pub fn sig(&self) -> &T {
        if let Data::Sig(t) = self { &t } else { panic!("This can't happen but makes compiler happy") }
    }
}

pub enum SyncData<T: Send + Sync> {
    Sig(T),
}

impl<T: Send + Sync> SyncData<T> {
    #[allow(irrefutable_let_patterns)]
    pub fn sig(&self) -> &T {
        if let SyncData::Sig(t) = self { &t } else { panic!("This can't happen but makes compiler happy") }
    }
}

pub trait Signal<K, U> {
    fn connect<T>(&mut self, slot: Rc<RefCell<dyn Slot<U>>>);
    fn emit(&self, event: Data<K>) -> Result<(), &str>;
}

pub trait SyncSignal<K: Send + Sync, U: Send>: Send + Sync {
    fn connect<T: Send + Sync>(&mut self, slot: Arc<RwLock<dyn SyncSlot<U>>>);
    fn emit(&self, event: SyncData<K>) -> Result<(), &str>;
}

pub trait Slot<T> {
    fn consume(&mut self, event: &Data<&T>) -> bool;

    fn as_any(&self) -> &dyn Any;
}

pub trait SyncSlot<T: Send + Sync>: Send + Sync {
    fn consume(&mut self, event: &SyncData<&T>) -> bool;

    fn as_any(&self) -> &dyn Any;
}
