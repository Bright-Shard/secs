mod archetype;
mod entity;
mod resource;
mod system;
mod world;

use world::World;

pub mod prelude {
    pub use crate::{
        entity::{Component, EntityBuilder},
        system::{Query, Resource},
        world::World,
    };
}
