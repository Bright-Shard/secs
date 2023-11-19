use {
    crate::{_crate_prelude::*, entity::ComponentBorrow, system::WorldData},
    alloc::rc::Rc,
    core::{
        cell::{Ref, RefCell, RefMut},
        marker::PhantomData,
    },
};

/// Queries give systems access to entities' components in the world.
pub struct Query<B: Queryable> {
    bundles: Vec<(usize, Vec<Rc<RefCell<dyn Component>>>)>,
    _bundle_ty: PhantomData<B>,
}
impl<Q: Queryable> Query<Q> {
    /// Iterates over all of the queried components, one entity at a time.
    pub fn iter(&self) -> QueryIter<'_, Q> {
        QueryIter {
            query: self,
            idx: 0,
        }
    }
    /// Iterates over all of the queried components, and the entities those components
    /// belong to. In other words, it iterates over `(entity, component(s))`, where `entity`
    /// is a `usize` (the entity's ID) and `component(s)` is the component or tuple of components
    /// that were actually queried.
    pub fn iter_with_entity(&self) -> QueryEntityIter<'_, Q> {
        QueryEntityIter {
            query: self,
            idx: 0,
        }
    }

    /// The total number of entities that satisfied this query.
    pub fn len(&self) -> usize {
        self.bundles.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bundles.len() == 0
    }
}

/// Allow `Query`s to be used as system parameters.
impl<Q: Queryable + 'static> WorldData for Query<Q> {
    fn take(world: &mut World) -> Self {
        let types = Q::type_ids();
        let num_types = types.len();
        let mut archetypes = Vec::with_capacity(num_types);

        for ty in types {
            archetypes.push(
                world
                    .storage
                    .get_archetype(ty)
                    .expect("TODO: Error handling. Query failed to get archetype it needed."),
            );
        }

        let mut bundles = Vec::new();
        let mut idx = 0;
        let max = world.storage.num_entities;

        'bundles: while idx < max {
            let mut cache = Vec::with_capacity(num_types);
            for archetype in archetypes.iter() {
                if let Some(component) = archetype.borrow_mut().get_component(idx) {
                    cache.push(component)
                } else {
                    idx += 1;
                    continue 'bundles;
                }
            }
            bundles.push((idx, cache));

            idx += 1;
        }

        Self {
            bundles,
            _bundle_ty: PhantomData,
        }
    }

    fn release(self, _world: &mut World) {}
}

/// An iterator for `Query`s. Iterates over components from the query.
pub struct QueryIter<'a, B: Queryable> {
    query: &'a Query<B>,
    idx: usize,
}
impl<'a, Q: Queryable> Iterator for QueryIter<'a, Q> {
    type Item = Q::QueryResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.query.bundles.len() {
            return None;
        }

        self.idx += 1;
        Some(Q::from_components(
            &mut self.query.bundles[self.idx - 1].1.iter(),
        ))
    }
}

/// An iterator for `Query`s. Iterates over components from the query, and their entities.
pub struct QueryEntityIter<'a, Q: Queryable> {
    query: &'a Query<Q>,
    idx: usize,
}
impl<'a, Q: Queryable> Iterator for QueryEntityIter<'a, Q> {
    type Item = (usize, Q::QueryResult<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.query.bundles.len() {
            return None;
        }

        let (entity, components) = &self.query.bundles[self.idx];
        self.idx += 1;
        Some((entity.clone(), Q::from_components(&mut components.iter())))
    }
}

impl<'a, Q: Queryable> IntoIterator for &'a Query<Q> {
    type Item = Q::QueryResult<'a>;
    type IntoIter = QueryIter<'a, Q>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, Q: Queryable> IntoIterator for &'a mut Query<Q> {
    type Item = Q::QueryResult<'a>;
    type IntoIter = QueryIter<'a, Q>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Any type that's valid to be used in a `Query`.
pub trait Queryable {
    /// The type that will be returned from querying with this.
    type QueryResult<'a>;

    /// The `TypeId`s for the components this query needs.
    fn type_ids() -> Vec<TypeId>;

    /// Build the query from the components it needs.
    fn from_components<'a>(
        components: &mut impl Iterator<Item = &'a Rc<RefCell<dyn Component>>>,
    ) -> Self::QueryResult<'a>;
}
impl<'b, A: Component> Queryable for &'b A {
    type QueryResult<'a> = Ref<'a, A>;

    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<A>()]
    }

    fn from_components<'a>(
        components: &mut impl Iterator<Item = &'a Rc<RefCell<dyn Component>>>,
    ) -> Self::QueryResult<'a> {
        Ref::map(components.next().unwrap().borrow(), |component| {
            component.as_any_ref().downcast_ref().unwrap()
        })
    }
}
impl<'b, A: Component> Queryable for &'b mut A {
    type QueryResult<'a> = RefMut<'a, A>;

    fn type_ids() -> Vec<TypeId> {
        vec![TypeId::of::<A>()]
    }

    fn from_components<'a>(
        components: &mut impl Iterator<Item = &'a Rc<RefCell<dyn Component>>>,
    ) -> Self::QueryResult<'a> {
        RefMut::map(components.next().unwrap().borrow_mut(), |component| {
            component.as_any_mut().downcast_mut().unwrap()
        })
    }
}
macro_rules! queryable_impl {
    ($_ty:ident) => {

    };
    ($_ty:ident $($ty:ident)*) => {
        impl <$($ty: Queryable + ComponentBorrow,)*> Queryable for ($($ty,)*) {
            type QueryResult<'a> = ($($ty::QueryResult<'a>,)*);

            fn type_ids() -> Vec<TypeId> {
                vec![$(TypeId::of::<$ty::Component>(),)*]
            }

            fn from_components<'a>(
                components: &mut impl Iterator<Item = &'a Rc<RefCell<dyn Component>>>,
            ) -> Self::QueryResult<'a> {
                ($($ty::from_components(components),)*)
            }
        }

        queryable_impl!($($ty)*);
    };
}
queryable_impl!(A A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

#[cfg(test)]
mod tests {
    use {super::*, crate as secs, secs_macros::Component};

    #[derive(Component)]
    struct MyComponent {}
    #[derive(Component)]
    struct OtherComponent {}

    fn accepts_queryable(_: impl Queryable) {}

    #[test]
    /// Verifies that a `Query` accepts component references and tuples of component references.
    /// This is only testing the type system, so as long as it compiles it's passing.
    fn query_type() {
        // Verify queries can take 1 component, multiple components, mutable borrows, and immutable borrows.
        accepts_queryable(&MyComponent {});
        accepts_queryable(&mut MyComponent {});
        accepts_queryable((&mut MyComponent {}, &MyComponent {}));

        // Verify this also works for systems.
        fn _system(_: &Query<&MyComponent>) {}
        fn _system2(_: Query<&mut MyComponent>) {}
        fn _system3(_: Query<(&mut MyComponent,)>) {}
        fn _system4(_: Query<(&MyComponent,)>) {}
        fn _system5(_: Query<(&MyComponent, &OtherComponent)>) {}
        fn _system6(_: Query<(&mut MyComponent, &OtherComponent)>) {}
        fn _system7(_: Query<(&MyComponent, &mut OtherComponent)>) {}
        fn _system8(_: Query<(&mut MyComponent, &mut OtherComponent)>) {}

        // This should fail when uncommented, since it's a component and not a component ref
        // fn _system0(_: Query<MyComponent>) {}
    }
}
