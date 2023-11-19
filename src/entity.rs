use crate::_crate_prelude::*;

/// Methods that make handling Components easier. These are automatically implemented with
/// `#[derive(Component)]`.
pub trait Component: AsAny {
    /// Prepares the `World`'s storage to store this `Component`.
    ///
    /// Why is this necessary? It's because there's no way to access
    /// a component's actual type from its trait object. However,
    /// we need its type to be able to make a vector (or other storage
    /// system) for the component. Letting the trait implementation
    /// handle this solves the issue, since the impl has direct access
    /// to the component's type.
    fn prep_storage(&self, storage: &mut Storage);
}

/// An immutable or mutable borrow of a component.
pub trait ComponentBorrow {
    /// The component that's being borrowed: The `C` in `&C` or `&mut C`.
    type Component: Component;
}
impl<'a, C: Component> ComponentBorrow for &'a C {
    type Component = C;
}
impl<'a, C: Component> ComponentBorrow for &'a mut C {
    type Component = C;
}

/// A tuple of components. Typically used to spawn entities.
pub trait Bundle {
    /// Consumes the bundle, returning a vec of the components it contained.
    fn components(self) -> Vec<Box<dyn Component>>;
    /// Same as above, but for a boxed bundle. Unsized types can't be moved,
    /// so in the case of a `Box<dyn Bundle>`, we need a special method that
    /// doesn't move out of the box.
    fn components_from_box(self: Box<Self>) -> Vec<Box<dyn Component>>;
}

impl<C: Component> Bundle for C {
    fn components(self) -> Vec<Box<dyn Component>> {
        vec![Box::new(self)]
    }

    fn components_from_box(self: Box<Self>) -> Vec<Box<dyn Component>> {
        vec![self]
    }
}
macro_rules! bundle_impl {
    ($_: ident) => {};
    ($_:ident $($ty:ident)*) => {
        secs_macros::impl_bundle!($($ty)*);

        bundle_impl!($($ty)*);
    };
}
bundle_impl!(A A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

#[cfg(test)]
mod tests {
    use {super::*, crate as secs, secs_macros::Component};

    #[derive(Component)]
    struct MyComponent {}
    #[derive(Component)]
    struct OtherComponent {}

    fn accepts_bundle(_: impl Bundle) {}

    /// This tests that a function accepting a `Bundle` object can accept a tuple of components.
    /// It's just testing the type system, so if it compiles, it's passing.
    #[test]
    fn bundle_type_test() {
        // Make sure Bundle accepts a single component
        accepts_bundle(MyComponent {});

        // Make sure Bundle accepts multiple components
        accepts_bundle((MyComponent {},));
        accepts_bundle((MyComponent {}, OtherComponent {}));
    }
}
