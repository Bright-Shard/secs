use {crate::world::World, std::any::Any};

/// Trait for all components that make up UI items
pub trait Component {
    fn prep_archetype(&self, world: &mut World);
    fn as_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Builder struct for making entities
#[derive(Default)]
pub struct EntityBuilder {
    components: Vec<Box<dyn Component>>,
}
impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component<C: Component + 'static>(mut self, component: C) -> Self {
        self.components.push(Box::new(component));
        self
    }

    pub fn build(self, world: &mut World) -> usize {
        let id = world.new_entity();
        for component in self.components {
            component.prep_archetype(world);
            world
                .insert_component_unchecked(id, component.as_any())
                .expect(concat!(
                    "There was an error building the EntityBuilder. ",
                    "This is probably a bug with SECS; please open an issue on GitHub."
                ));
        }
        id
    }
}
