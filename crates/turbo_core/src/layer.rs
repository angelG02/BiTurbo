use std::collections::HashMap;

pub use turbo_window::Event;

pub trait Layer {
    fn on_attach(&self);
    fn on_detach(&self);
    fn on_tick(&self, delta_time: f32);
    fn on_event(&self, event: &Event);
}

/// Structure responsible for managing layers and overlays
///
/// Layers are inserted in the front of the vector.
/// Overlays are pushed back
pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    names: HashMap<String, usize>,
    insert_index: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        LayerStack {
            layers: vec![],
            names: HashMap::new(),
            insert_index: 0,
        }
    }

    pub fn push_layer(&mut self, layer_name: &str, layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layers.insert(self.insert_index, layer);
        self.names.insert(layer_name.to_owned(), self.insert_index);
        self.insert_index += 1;
    }

    pub fn pop_layer(&mut self, layer_name: &str) {
        if let Some(index) = self.names.remove(layer_name) {
            if index < self.layers.len() {
                let layer = self.layers.remove(index);
                layer.on_detach();
                self.insert_index -= 1;
            }
        }
    }

    /// Overlays will always be pushed to the back of the Layer Stack (Will always be on top of the layers)
    pub fn push_overlay(&mut self, overlay_name: &str, overlay: Box<dyn Layer>) {
        overlay.on_attach();
        self.layers.push(overlay);
        self.names
            .insert(overlay_name.to_owned(), self.layers.len() - 1);
    }

    pub fn pop_overlay(&mut self, overlay_name: &str) {
        if let Some(index) = self.names.remove(overlay_name) {
            if index < self.layers.len() {
                let layer = self.layers.remove(index);
                layer.on_detach();
            }
        }
    }
}

impl Drop for LayerStack {
    fn drop(&mut self) {
        for layer in &self.layers {
            layer.on_detach();
        }
    }
}

impl<'a> IntoIterator for &'a LayerStack {
    type Item = &'a dyn Layer;
    type IntoIter = LayerStackIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LayerStackIterator {
            layer_stack: self,
            current_layer: 0,
            reverse: false,
        }
    }
}

impl<'a> IntoIterator for &'a mut LayerStack {
    type Item = &'a mut dyn Layer;
    type IntoIter = LayerStackMutIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LayerStackMutIterator {
            layer_stack: self,
            current_layer: 0,
            reverse: false,
        }
    }
}

pub struct LayerStackIterator<'a> {
    layer_stack: &'a LayerStack,
    current_layer: usize,
    reverse: bool,
}

impl<'a> Iterator for LayerStackIterator<'a> {
    type Item = &'a dyn Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_layer < self.layer_stack.layers.len() {
            let layer = &self.layer_stack.layers[self.current_layer];
            self.current_layer += 1;
            Some(layer.as_ref())
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for LayerStackIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_layer == 0 && !self.reverse {
            self.reverse = true;
            self.current_layer = self.layer_stack.layers.len();
        }
        if self.current_layer > 0 {
            self.current_layer -= 1;
            let layer = &self.layer_stack.layers[self.current_layer];
            Some(layer.as_ref())
        } else {
            None
        }
    }
}

pub struct LayerStackMutIterator<'a> {
    layer_stack: &'a mut LayerStack,
    current_layer: usize,
    reverse: bool,
}

impl<'a> Iterator for LayerStackMutIterator<'a> {
    type Item = &'a mut dyn Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_layer < self.layer_stack.layers.len() {
            unsafe {
                let ptr = &mut self.layer_stack.layers.as_mut_ptr();
                let layer = (*ptr.add(self.current_layer)).as_mut();
                self.current_layer += 1;
                Some(layer)
            }
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for LayerStackMutIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_layer == 0 && !self.reverse {
            self.reverse = true;
            self.current_layer = self.layer_stack.layers.len();
        }
        if self.current_layer > 0 {
            self.current_layer -= 1;
            unsafe {
                let ptr = &mut self.layer_stack.layers.as_mut_ptr();
                let layer = (*ptr.add(self.current_layer)).as_mut();
                Some(layer)
            }
        } else {
            None
        }
    }
}
