use crate::world::World;

pub trait System {
    fn execute(&self, world: &mut World);
}
pub trait IntoSystem<Result: System> {
    fn into_system(self) -> Result;
}

// region: SystemStore

pub struct SystemStore<Params>(Box<dyn SystemParamFn<Params>>);
impl<Params> System for SystemStore<Params> {
    fn execute(&self, world: &mut World) {
        self.0.execute(world);
    }
}

// endregion: SystemStore

// region: SystemParamFn

pub trait SystemParamFn<Params> {
    fn execute(&self, world: &mut World);
}

macro_rules! impl_system_param_fn {
    ($_: ident) => {};
    ($_: ident $($x: ident)*) => {
        secs_derive::impl_system_param_fn!($($x)*);
        impl_system_param_fn!($($x)*);
    };
}
impl_system_param_fn!(A A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

impl<F, Params> IntoSystem<SystemStore<Params>> for F
where
    F: SystemParamFn<Params> + 'static,
{
    fn into_system(self) -> SystemStore<Params> {
        SystemStore(Box::new(self))
    }
}

// endregion: SystemParamFn

// region: SystemParam

/// Data that can be taken from and returned to a World
pub trait WorldData: 'static {
    /// Releases any taken data back into the World
    fn release(self, world: &mut World);

    /// Takes data from the World, creating a new instance of itself
    fn take(world: &mut World) -> Self;
}

pub trait SystemParam {
    type Data: WorldData;
    type Fetch<'a>: SystemParam;

    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_>;
}

impl<WD: WorldData> SystemParam for &'_ WD {
    type Data = WD;
    type Fetch<'a> = &'a WD;

    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_> {
        &(*data)
    }
}
impl<WD: WorldData> SystemParam for &'_ mut WD {
    type Data = WD;
    type Fetch<'a> = &'a mut WD;

    fn fetch(data: &mut Self::Data) -> Self::Fetch<'_> {
        data
    }
}

// endregion: SystemParam

// region: Systems

#[derive(Default)]
pub struct Systems(Vec<Box<dyn System>>);
impl Systems {
    pub fn run(&self, world: &mut World) {
        for system in &self.0 {
            system.execute(world);
            world.apply_commands();
        }
    }

    pub fn push<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
        self.0.push(Box::new(system.into_system()));
    }
}

// endregion: Systems
