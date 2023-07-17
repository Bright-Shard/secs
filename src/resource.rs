use {
    crate::{system::WorldData, world::World},
    std::any::Any,
};

/// A resource stored in the world
pub struct Resource<R: 'static>(pub R);

/// Type-erased access to a resource stored in the world
pub trait AnonymousWorldResource {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_owned(self: Box<Self>) -> Box<dyn Any>;
}

impl<D: 'static> AnonymousWorldResource for Resource<D> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn as_any_owned(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}

impl<R: 'static> WorldData for Resource<R> {
    fn take(world: &mut World) -> Self {
        world.take_resource::<R>().expect("Bruh (resource edition)")
    }

    fn release(self, world: &mut World) {
        world.return_resource(self);
    }
}
impl<R: 'static> Resource<R> {
    pub fn get(&self) -> &R {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.0
    }
}
