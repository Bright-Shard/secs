mod archetype;
mod resource;
mod system;
mod world;

use {std::any::Any, system::Systems, world::World};

/// Trait for all components that make up UI items
pub trait Component {
    /// Lets the component store the entity it belongs to.
    /// Called automatically by the [`Widget.register_component()`]
    ///
    /// [`Widget.register_component()`](Widget::register_component)
    fn set_entity(&mut self, entity: usize);

    /// Convert to std::Any
    fn to_any(self) -> Box<dyn Any>;
}

pub mod prelude {
    pub use crate::{
        system::{Query, Resource},
        world::World,
    };
}
