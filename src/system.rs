//! Traits for systems in SECS, as well as traits for their parameters, and
//! the `Systems` type that actually runs those systems.
//!
//! This code currently relies on a bunch of traits and is quite messy. It will
//! probably be modified in the future to remove some of the traits, if possible.
//! Currently, it works like this:
//!
//! Arguments systems can use all implement the `SystemParam` trait. This trait
//! takes some `WorldData` (any data that can be taken from the world and returned
//! later) to create itself, after which it's given to the system to be run.
//!
//! Systems use 3 traits and a struct: `System`, a trait with an `execute` function
//! that actually runs the system; `SystemStore`, a struct that stores a system, and
//! implements `System` so it can run it; `SystemParamFn`, a trait implemented for
//! any functions that have only `SystemParam`s as arguments; and `IntoSystem`, a trait
//! that stores `SystemParamFn`s inside `SystemStores` so they can be executed as `System`s.
//!
//! This setup is quite complicated, but is necessary for type-erasure and lifetime-erasure.
//! Functions have a lot of generics, for their arguments, and lifetimes associated
//! with those arguments. By chaining these traits and struct together, SECS is able to
//! erase those types and lifetimes into a single `System` trait object with an `execute`
//! method that runs the function. In the future, it may be possible to accomplish this
//! with less traits and a more organised system, and SECS will definitely switch then if
//! possible.

use crate::_crate_prelude::*;

pub mod command;
pub mod query;
pub mod resource;

/// The base trait for all `System`s, which just allows them to be executed
/// with mutable access to the world. Getting the system's parameters and data
/// is left up to the trait impl.
pub trait System {
    fn execute(&self, world: &mut World);
}

/// A struct that stores a system. This is actually the only type that implements
/// `System`, because it is able to type-erase a lot of the function's generics.
pub struct SystemStore<Params>(Box<dyn SystemParamFn<Params>>);
impl<Params> System for SystemStore<Params> {
    fn execute(&self, world: &mut World) {
        self.0.execute(world);
    }
}

/// A function with parameters that implement `SystemParam`.
pub trait SystemParamFn<Params> {
    fn execute(&self, world: &mut World);
}
macro_rules! impl_system_param_fn {
    ($_unused: ident) => {};
    ($_unused: ident $($x: ident)*) => {
        secs_macros::impl_system_param_fn!($($x)*);
        impl_system_param_fn!($($x)*);
    };
}
impl_system_param_fn!(A A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

/// A trait to store `SystemParamFn`s in `SystemStore`s, so they can be
/// executed like regular systems.
pub trait IntoSystem<Result: System> {
    fn into_system(self) -> Result;
}
impl<F, Params> IntoSystem<SystemStore<Params>> for F
where
    F: SystemParamFn<Params> + 'static,
{
    fn into_system(self) -> SystemStore<Params> {
        SystemStore(Box::new(self))
    }
}

/// Data that can be taken from and returned to the `World`. System parameters use this trait.
pub trait WorldData: 'static {
    /// Takes data from the `World` to create the data.
    fn take(world: &mut World) -> Self;

    /// Releases any taken data back into the World.
    fn release(self, world: &mut World);
}

/// A parameter that may be used in a system. Types that implement this can be used as arguments
/// in systems.
pub trait SystemParam {
    /// Data needed to create this `SystemParam`.
    type Data: WorldData;
    /// The output this `SystemParam` makes. This is essentially `Self`, but with a
    /// different lifetime that will match the system/function's lifetime.
    type Fetch<'a>;

    /// Takes the `SystemParam`'s data to make the `SystemParam`.
    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_>;
}

impl<WD: WorldData> SystemParam for &WD {
    type Data = WD;
    type Fetch<'a> = &'a WD;

    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_> {
        &(*data)
    }
}
impl<WD: WorldData> SystemParam for &mut WD {
    type Data = WD;
    type Fetch<'a> = &'a mut WD;

    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_> {
        data
    }
}

/// A struct that stores the World's Systems and runs them.
#[derive(Default)]
pub struct Systems(Vec<Box<dyn System>>);
impl Systems {
    /// Run every system once.
    pub fn run(&self, world: &mut World) {
        for system in &self.0 {
            system.execute(world);
        }
    }

    /// Add a new system to run.
    pub fn push<Sys: System + 'static>(&mut self, system: impl IntoSystem<Sys>) {
        self.0.push(Box::new(system.into_system()));
    }
}
