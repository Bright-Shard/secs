//! # SECS: The Smol ECS Crate
//! TODO: description
//!
//! # Reading the Source Code
//! Quick note: SECS doesn't hide anything in the documentation. Everything in the source code
//! should be visible here, and if it isn't, it's considered a bug. Furthermore, every part of
//! SECS is accessible and hackable; if for some god-forsaken reason you want to manually
//! add and remove archetypes from the world, you can do that,
//!
//! The code is laid out in a very specific way in SECS. Here's the current list of modules, and
//! what you can expect to find in them.
//! - `entity`: Defines entities, components, and bundles in SECS.
//! - `world`: Defines SECS' world, and the archetypes that store components.
//! - `system`: Defines systems, and basically the whole system API - queries, resources, etc...
//!
//! It's also worth noting that SECS does *not* use `mod.rs`. For example, the world module has a
//! `world.rs`, then a folder called `world`, and files in that folder for all its submodules. There
//! is no `world/mod.rs`. This is intentional, to keep alphabetical ordering and provide easier
//! searching.

#![no_std]
extern crate alloc;

pub mod entity;
pub mod system;
pub mod world;

pub(crate) mod _crate_prelude {
    pub use super::{
        entity::Component,
        world::{
            storage::{Archetype, Storage},
            World,
        },
        AsAny,
    };

    pub use alloc::{boxed::Box, vec, vec::Vec};
    pub use core::any::{Any, TypeId};
}

pub mod prelude {
    pub use crate::{
        entity::Component,
        system::{
            command::{Command, CommandQueue},
            query::Query,
            resource::Resource,
        },
        world::World,
    };
    pub use secs_macros::Component;
}

use _crate_prelude::*;
/// In many cases, SECS needs to temporarily cast a type to `Any`, so it can be downcasted later.
/// Upcasting is unstable, so currently the best way to do this is with trait methods that cast
/// self to `Any`. This trait just makes a global way to do that.
pub trait AsAny: Any {
    fn as_any_ref(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_owned(self: Box<Self>) -> Box<dyn Any>;
}
impl<T: Any> AsAny for T {
    fn as_any_ref(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_any_owned(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}
