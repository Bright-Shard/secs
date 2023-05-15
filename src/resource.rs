use std::any::Any;

pub struct WorldResource<D: 'static>(pub D);

pub trait AnonymousWorldResource {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_owned(self: Box<Self>) -> Box<dyn Any>;
}

impl<D: 'static> AnonymousWorldResource for WorldResource<D> {
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
