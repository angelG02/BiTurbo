use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::Component;
pub struct World {
    pub registry: HashMap<u32, Vec<Box<dyn Component>>>,
    id_counter: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn add_entity(&mut self) -> u32 {
        self.id_counter += 1;

        let component_vec: Vec<Box<dyn Component>> = Vec::new();

        self.registry.insert(self.id_counter, component_vec);

        return self.id_counter;
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.registry.remove(&entity_id);
    }

    pub fn add_component_by_entity_id<T: Component + 'static>(
        &mut self,
        entity_id: &u32,
        component: T,
    ) {
        if let Some(components) = self.registry.get_mut(&entity_id) {
            if !components
                .iter()
                .any(|c| c.as_any().type_id() == component.type_id())
            {
                components.push(Box::new(component));
            }
        }
    }

    pub fn remove_component_by_entity_id<T: Component + 'static>(&mut self, entity_id: &u32) {
        if let Some(components) = self.registry.get_mut(entity_id) {
            if let Some(comp_index) = components
                .iter()
                .position(|c| c.as_any().type_id() == TypeId::of::<T>())
            {
                components.swap_remove(comp_index);
            }
        }
    }

    pub fn get_all_components_of_type<T: Component + 'static>(&mut self) -> Option<Vec<&T>> {
        let mut desired_components: Vec<&T> = Vec::new();
        let type_id = TypeId::of::<T>();
        for comps in self.registry.values() {
            for comp in comps {
                if comp.as_any().type_id() == type_id {
                    if let Some(c) = comp.as_any().downcast_ref::<T>() {
                        desired_components.push(c);
                    }
                }
            }
        }

        return Some(desired_components);
    }

    pub fn get_all_entities_with_component<T: Component + 'static>(&self) -> Option<Vec<&u32>> {
        let mut entities: Vec<&u32> = Vec::new();
        for (entity, componments) in &self.registry {
            for comp in componments {
                if comp.as_any().type_id() == TypeId::of::<T>() {
                    entities.push(entity);
                }
            }
        }

        return Some(entities);
    }

    pub fn get_component_by_entity_id<T: Component + 'static>(
        &mut self,
        entity_id: &u32,
    ) -> Option<&T> {
        if let Some(components) = self.registry.get(&entity_id) {
            for component in components.iter() {
                if let Some(c) = component.as_any().downcast_ref::<T>() {
                    return Some(c);
                }
            }
        }

        None
    }
}
