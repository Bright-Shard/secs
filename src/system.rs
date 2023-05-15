use {
    crate::{archetype::Archetype, resource::WorldResource, Component, World},
    std::marker::PhantomData,
};

pub trait System {
    fn execute(&self, world: &mut World);
}
pub trait IntoSystem<Result: System> {
    fn into_system(self) -> Result;
}

// region: SystemStore

pub trait SystemParamFn<Params> {
    fn execute(&self, world: &mut World);
}
pub struct SystemStore<Params>(Box<dyn SystemParamFn<Params>>);
impl<Params> System for SystemStore<Params> {
    fn execute(&self, world: &mut World) {
        self.0.execute(world);
    }
}

impl<F, A> SystemParamFn<A> for F
where
    F: Fn(&A),
    A: SystemParam + Sized,
{
    fn execute(&self, world: &mut World) {
        let a = A::new(world);
        self(&a);
        a.release(world);
    }
}

impl<F, Params> IntoSystem<SystemStore<Params>> for F
where
    F: SystemParamFn<Params> + 'static,
{
    fn into_system(self) -> SystemStore<Params> {
        SystemStore(Box::new(self))
    }
}

// endregion: SystemStore

// region: SystemParam

pub trait SystemParam {
    fn new(world: &mut World) -> Self;
    fn release(self, world: &mut World);
}

// endregion: SystemParam

// region: Query

pub struct Query<C: Component + 'static>(Archetype<C>);
impl<C: Component + 'static> SystemParam for Query<C> {
    fn new(world: &mut World) -> Self {
        Self(world.take_archetype::<C>().unwrap())
    }
    fn release(self, world: &mut World) {
        world.return_archetype(self.0);
    }
}

// endregion: Query

// region: Resource

pub struct Resource<R: 'static>(WorldResource<R>);
impl<R: 'static> SystemParam for Resource<R> {
    fn new(world: &mut World) -> Self {
        // TODO: Don't unwrap the option from taking resources & archetypes
        // Also applies to Query above^^^
        Self(world.take_resource::<R>().unwrap())
    }
    fn release(self, world: &mut World) {
        world.return_resource(self.0);
    }
}
impl<R: 'static> Resource<R> {
    pub fn get(&self) -> &R {
        &self.0 .0
    }
}

// endregion: Resource

// region: Systems

#[derive(Default)]
pub struct Systems(Vec<Box<dyn System>>);
impl Systems {
    pub fn run(&self, world: &mut World) {
        for system in &self.0 {
            system.execute(world);
        }
    }

    pub fn push<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
        self.0.push(Box::new(system.into_system()));
    }
}

// endregion: Systems
