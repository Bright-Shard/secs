mod archetype;
mod command_queue;
mod entity;
mod resource;
mod system;
mod world;

pub mod prelude {
    pub use crate::{
        archetype::Query,
        command_queue::{Command, CommandQueue},
        entity::{Component, EntityBuilder},
        resource::Resource,
        world::World,
    };
    pub use secs_derive::Component;
}
