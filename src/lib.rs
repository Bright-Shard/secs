mod archetype;
mod entity;
mod resource;
mod system;
mod world;

pub mod prelude {
    pub use crate::{
        entity::{Component, EntityBuilder},
        system::{Query, Resource},
        world::World,
    };
}
