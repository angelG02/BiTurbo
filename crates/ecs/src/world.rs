use std::{collections::HashMap, any::{Any}};

use crate::Component;

pub struct World {
    pub registry: HashMap<u32, Vec<Box<dyn Component>>>,
    id_counter: u32
}

impl World {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            id_counter: 0
        }
    }

    // pub fn add_entity(&mut self, position_component: Position) {
    //     let comp = Box::new(position_component);
    //     let component_vec: Vec<Box<dyn Component>> = Vec::new();

    //     self.registry.insert(self.id_counter, component_vec);

    //     self.id_counter += 1;

    // }

    pub fn add_entity<>(&mut self) -> u32 {

        self.id_counter += 1;

        let component_vec: Vec<Box<dyn Component>> = Vec::new();

        self.registry.insert(self.id_counter, component_vec);

    
        return self.id_counter;

    }

    pub fn add_component_by_entity_id<T: Component + 'static>(&mut self, entity_id: u32, component: T) {

       if let Some(components) = self.registry.get_mut(&entity_id)
       {
            if components.iter().any(|c| c.as_ref().type_id() != component.type_id())
            {
                components.push(Box::new(component));
            }
       }       
    }

    // pub fn get_component_by_entity_id(&self, entity_id: u32, component: Box<dyn Component>) {

    // }

}