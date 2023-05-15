use {
    crate::world::World,
    std::any::{Any, TypeId},
};

/// Trait for all components that make up UI items
pub trait Component {}

/// Builder struct for making entities

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    components: Vec<Box<dyn Any>>,
}
impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self {
            world,
            components: Vec::new(),
        }
    }

    pub fn add_component<C: Component + 'static>(mut self, component: C) -> Self {
        self.world.prep_archetype::<C>();
        self.components.push(Box::new(component) as Box<dyn Any>);
        self
    }

    pub fn build(self) -> Result<(), ()> {
        let id = self.world.new_entity();
        for component in self.components {
            self.world.insert_component_unchecked(id, component)?;
        }
        Ok(())
    }
}
