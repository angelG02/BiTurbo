use std::{any::TypeId, collections::HashMap, fs::File, io::BufReader};

use crate::Component;

use serde::{Deserialize, Serialize};
//use serde_json::json;
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub registry: HashMap<u32, Vec<Box<dyn Component>>>,
    id_counter: u32,
}

impl World {
    /// Constructs a new empty `Registry` instance.
    ///
    /// This function creates a new instance of `Registry` with an empty `HashMap` to store entities and their
    /// associated component vectors, and an initial ID counter of 0.
    ///
    /// Returns the newly created `Registry` instance.
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            id_counter: 0,
        }
    }

    /// Adds a new entity to the registry with a unique ID and an empty component vector.
    ///
    /// This function increments the `id_counter` of the registry, creates an empty component vector, and
    /// associates the new entity ID with the component vector in the registry.
    ///
    /// Returns the ID of the newly added entity.
    pub fn add_entity(&mut self) -> u32 {
        self.id_counter += 1;

        let component_vec: Vec<Box<dyn Component>> = Vec::new();

        self.registry.insert(self.id_counter, component_vec);

        return self.id_counter;
    }

    /// Removes an entity with the given ID from the registry, if it exists.
    ///
    /// This function removes the entity with the given ID and its associated component vector from the registry, if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity to remove.
    pub fn remove_entity(&mut self, entity_id: u32) {
        self.registry.remove(&entity_id);
    }

    /// Adds a component to the entity with the given ID, if it doesn't already have a component of the same type.
    ///
    /// This function adds a component of the given type to the entity with the given ID, if the entity does not already
    /// have a component of the same type. If the entity does have a component of the same type, nothing is added.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to add. Must implement the `Component` trait and outlive the function.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity to which the component will be added.
    /// * `component` - The component to add to the entity.
    pub fn add_component<T: Component + 'static>(&mut self, entity_id: &u32, component: T) {
        if let Some(components) = self.registry.get_mut(&entity_id) {
            if !components
                .iter()
                .any(|c| c.as_any().type_id() == component.type_id())
            {
                components.push(Box::new(component));
            }
        }
    }

    /// Removes a component of the given type from the entity with the given ID, if it exists.
    ///
    /// This function removes a component of the given type from the entity with the given ID, if the entity has a
    /// component of that type. If the entity does not have a component of that type, nothing is removed.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to remove. Must implement the `Component` trait and outlive the function.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity from which the component will be removed.

    pub fn remove_component<T: Component + 'static>(&mut self, entity_id: &u32) {
        if let Some(components) = self.registry.get_mut(entity_id) {
            if let Some(comp_index) = components
                .iter()
                .position(|c| c.as_any().type_id() == TypeId::of::<T>())
            {
                components.swap_remove(comp_index);
            }
        }
    }

    /// Gets all components of the given type from all entities in the registry, if any exist.
    ///
    /// This function returns a vector containing all components of the given type from all entities in the registry,
    /// if any such components exist. If no components of the given type exist in the registry, this function returns
    /// `None`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to retrieve. Must implement the `Component` trait and outlive the function.
    ///
    /// # Returns
    ///
    /// A vector containing references to all components of the given type in the registry, if any exist. If no
    /// components of the given type exist in the registry, this function returns `None`.
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

    /// Gets the IDs of all entities that have a component of the given type, if any exist.
    ///
    /// This function returns a vector containing the IDs of all entities that have a component of the given type,
    /// if any such entities exist. If no entities in the registry have a component of the given type, this function
    /// returns `None`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to search for. Must implement the `Component` trait and outlive the function.
    ///
    /// # Returns
    ///
    /// A vector containing the IDs of all entities that have a component of the given type in the registry, if any
    /// such entities exist. If no entities in the registry have a component of the given type, this function returns
    /// `None`.
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

    /// Gets a reference to the component of the given type for the entity with the specified ID, if it exists.
    ///
    /// This function returns a reference to the component of the given type for the entity with the specified ID,
    /// if it exists. If no such component exists, this function returns `None`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to retrieve. Must implement the `Component` trait and outlive the function.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity to retrieve the component for.
    ///
    /// # Returns
    ///
    /// A reference to the component of the given type for the entity with the specified ID, if it exists. If no such
    /// component exists, this function returns `None`.
    pub fn get_component<T: Component + 'static>(&mut self, entity_id: &u32) -> Option<&T> {
        if let Some(components) = self.registry.get(&entity_id) {
            for component in components.iter() {
                if let Some(c) = component.as_any().downcast_ref::<T>() {
                    return Some(c);
                }
            }
        }

        None
    }

    /// Gets a mutable reference to the component of the given type for the entity with the specified ID, if it exists.
    ///
    /// This function returns a mutable reference to the component of the given type for the entity with the specified
    /// ID, if it exists. If no such component exists, this function returns `None`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of component to retrieve. Must implement the `Component` trait and outlive the function.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity to retrieve the component for.
    ///
    /// # Returns
    ///
    /// A mutable reference to the component of the given type for the entity with the specified ID, if it exists. If no
    /// such component exists, this function returns `None`.
    pub fn get_mut_component<T: Component + 'static>(&mut self, entity_id: &u32) -> Option<&mut T> {
        if let Some(components) = self.registry.get_mut(&entity_id) {
            for component in components.iter_mut() {
                if let Some(c) = component.as_any_mut().downcast_mut::<T>() {
                    return Some(c);
                }
            }
        }

        None
    }

    /// Checks if the entity with the given ID has a component of the specified type.
    ///
    /// # Arguments
    ///
    /// * entity_id - The ID of the entity to check for the component.
    ///
    /// # Type Parameters
    ///
    /// * T - The type of the component to check for.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the entity has a component of the specified type.
    pub fn has_component<T: Component + 'static>(&self, entity_id: &u32) -> bool {
        if let Some(components) = self.registry.get(&entity_id) {
            for component in components.iter() {
                if let Some(_c) = component.as_any().downcast_ref::<T>() {
                    return true;
                }
            }
        }

        return false;
    }

    /// Serialize the current world state to a JSON file.
    ///
    /// # Arguments
    ///
    /// * file_path - The path of the file where the world state will be serialized.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if the serialization is successful, and Err("The file does not exist") if the file creation fails.
    pub fn serialize(&self, file_path: &str) -> Result<(), &str> {
        if let Ok(file) = File::create(file_path) {
            let _json = serde_json::to_writer(file, &self).unwrap();
            //println!("This is world serialized: {:?}", json);
            return Ok(());
        } else {
            return Err("The file does not exist");
        }
    }

    /// Deserializes the contents of a file at the specified file path and updates
    /// this `World` instance with the deserialized data.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice containing the path to the file to be deserialized.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the deserialization process was successful and `Err(&str)`
    /// with an error message if the file could not be opened or deserialized.
    pub fn desirialize(&mut self, file_path: &str) -> Result<(), &str> {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            let _json: Result<World, serde_json::Error> = serde_json::from_reader(reader);
            //println!("This is world: {:?}", json);
            return Ok(());
        } else {
            return Err(
                "The file you tried to access does not exist, or the file path is incorrect",
            );
        }
    }
}
