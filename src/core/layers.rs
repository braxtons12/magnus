use std::slice::IterMut;

use crate::core::object::Object;
use crate::events::event::Event;

pub struct Layer {
	enabled: bool,
	objects: Vec<Box<dyn Object>>,
	debug_name: String,
}

impl Layer {
	pub fn new(enabled: bool, objects: Option<Vec<Box<dyn Object>>>, debug_name: String) -> Layer {
		Layer {
			enabled,
			objects: match objects {
				Some(x) => x,
				None    => Vec::new()
			},
			debug_name } 
	}

	#[inline]
	pub fn enabled(&self) -> bool {
		self.enabled
	}

	#[inline]
	pub fn set_enabled(&mut self, enable: bool) {
		self.enabled = enable;
	}

	#[inline]
	pub fn debug_name(&self) -> &String {
		&self.debug_name
	}

	#[inline]
	pub fn set_debug_name(&mut self, name: String) {
		self.debug_name = name;
	}

	pub fn push_object(&mut self, obj: Box<dyn Object>) {
		self.objects.push(obj);
	}

	pub fn insert_object(&mut self, obj: Box<dyn Object>, index: usize) {
		self.objects.insert(index, obj);
	}

	pub fn remove_object(&mut self, index: usize) -> Box<dyn Object> {
		self.objects.remove(index)
	}

	pub fn get_obj(&self, index: usize) -> &Box<dyn Object> {
		self.objects.get(index).expect("Index out of bounds!")
	}

	pub fn on_event(&mut self, e: &mut dyn Event) {

	}

	pub fn on_update(&mut self) {

	}
}

pub struct LayerStack {
	layers: Vec<Layer>,
	insert_index: usize
}

impl LayerStack {
	pub fn new(layers: Option<Vec<Layer>>, index: Option<usize>) -> LayerStack {
		match layers {
			Some(x) => LayerStack { layers: x, insert_index: match index {
				Some(y) => y,
				None    => panic!("Gave a prebuilt layers vector, but not an index for regular layers end")
			} },
			None    => LayerStack { layers: Vec::new(), insert_index: 0 }
		}
	}

	pub fn push_layer(&mut self, layer: Layer) {
		self.layers.insert(self.insert_index, layer);
		self.insert_index += self.insert_index;
	}

	pub fn push_overlay(&mut self, layer: Layer) {
		self.layers.push(layer);
	}

	pub fn remove_layer(&mut self) -> Layer {
		self.insert_index -= self.insert_index;
		self.layers.remove(self.insert_index)
	}

	pub fn remove_overlay(&mut self) -> Layer {
		self.layers.remove(self.layers.len() - 1)
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Layer> {
		self.layers.iter_mut()
	}
}
