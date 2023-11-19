use {
    crate::{_crate_prelude::*, system::WorldData},
    alloc::rc::Rc,
    core::{
        cell::{Ref, RefCell, RefMut},
        marker::PhantomData,
    },
};

/// A resource stored in the World.
pub struct Resource<R: 'static> {
    value: Rc<RefCell<dyn Any>>,
    r: PhantomData<R>,
}
/// Allow `Resource`s to be used as system parameters.
impl<R: 'static> WorldData for Resource<R> {
    fn take(world: &mut World) -> Self {
        Self {
            value: world
                .storage
                .get_resource(TypeId::of::<R>())
                .expect("TODO: Error handling, resource failed to get"),
            r: PhantomData,
        }
    }

    fn release(self, _world: &mut World) {}
}
impl<R: 'static> Resource<R> {
    /// Immutably get a resource's value.
    pub fn get(&self) -> Ref<'_, R> {
        Ref::map(self.value.borrow(), |resource| {
            resource.downcast_ref().unwrap()
        })
    }

    /// Mutably get a resource's value.
    pub fn get_mut(&self) -> RefMut<'_, R> {
        RefMut::map(self.value.borrow_mut(), |resource| {
            resource.downcast_mut().unwrap()
        })
    }
}
