pub use turbo_window::Event;

pub trait Layer {
    fn on_attach(&mut self);
    fn on_detach(&self);
    fn on_update(&self, delta_time: f32);
    fn on_event(&self, event: &Event);
}

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
}

impl LayerStack {
    pub fn new() -> Self {
        LayerStack { layers: vec![] }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn pop_layer(&mut self, _layer: Box<&dyn Layer>) {
        // if let Some(index) = self
        //     .layers
        //     .iter()
        //     .position(|l| l as *const dyn Layer == *layer as *const dyn Layer)
        // {
        //     self.layers.remove(index);
        // }
        todo!()

        // Data flow is weird...should the layer stack own the layers or hold a ref to them?
        // If it holds a ref then where are layers created? They cannot be created with a lifetime
        // that would be shorter than the one of the App (which holds the layer stack) but a layer
        // would need to be created in app right? Cherno -> ImGuiLayer is created on App::init
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

pub struct Kur {
    pub debug_name: String,
}

impl Layer for Kur {
    fn on_attach(&mut self) {
        println!("Name: {0}", self.debug_name);
    }

    fn on_detach(&self) {
        todo!()
    }

    fn on_event(&self, _event: &Event) {
        todo!()
    }

    fn on_update(&self, _delta_time: f32) {
        todo!()
    }
}
