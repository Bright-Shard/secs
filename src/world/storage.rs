//! The storage backend for SECS, which stores all of the resources and entities
//! in the world.
//!
//! It might make more sense to picture the World like this:
//!
//!```txt
//!                 Entity 1       Entity 2       Entity 3
//! Component1:   Some(<data>)       None           None
//! Component2:   Some(<data>)       None       Some(<data>)
//! Component3:   Some(<data>)   Some(<data>)       None
//! ```
//!
//! In this layout, the whole table is the World, each row is an Archetype, each column is an
//! Entity, and each table item is a Component.

use {
    crate::_crate_prelude::*,
    alloc::rc::Rc,
    core::{
        cell::RefCell,
        hash::{BuildHasher, Hasher},
        ops::Deref,
    },
    hashbrown::HashMap,
};

/// Archetypes store components in the world. There's an archetype for each
/// type of component in the world. Each archetype stores one type of component,
/// and stores that type for every single entity.
///
/// This is a trait for archetypes: the struct that actually stores components is
/// `WorldArchetype`. However, that struct is generic, so it can't be stored in
/// the world (each archetype has a different generic for the type of component
/// it stores). This trait is not generic - it "type-erases" the archetype, providing
/// useful functions for all archetypes without having to handle each individual
/// archetype's component type.
pub trait Archetype: AsAny {
    /// Update an entity's component.
    fn set(&mut self, entity: usize, component: Box<dyn Component>);
    /// Get an entity's component.
    fn get_component(&self, entity: usize) -> Option<Rc<RefCell<dyn Component>>>;

    /// Removes the component this archetype stores for an entity.
    fn despawn(&mut self, entity: usize);

    /// Tells the archetype a new entity has spawned, so it can allocate space for it.
    fn add_entity(&mut self);

    /// The `TypeId` of the component this Archetype stores.
    fn contained_type_id(&self) -> TypeId;
}

/// The struct that actually stores components for an archetype. See the `Archetype`
/// trait for more info.
#[derive(Debug)]
pub struct WorldArchetype<C: Component> {
    components: Vec<Option<Rc<RefCell<C>>>>,
}
impl<C: Component> WorldArchetype<C> {
    /// Make a new Archetype with a specific starting size.
    pub fn new_with_size(size: usize) -> Self {
        let mut components = Vec::with_capacity(size);
        (0..size).for_each(|_| components.push(None));

        Self { components }
    }
}
impl<C: Component> Archetype for WorldArchetype<C> {
    fn set(&mut self, entity: usize, component: Box<dyn Component>) {
        self.components[entity] = Some(Rc::new(RefCell::new(
            *component
                .as_any_owned()
                .downcast()
                .expect("TODO: Error handling. Component was inserted into the wrong archetype."),
        )))
    }
    fn get_component(&self, entity: usize) -> Option<Rc<RefCell<dyn Component>>> {
        self.components[entity]
            .as_ref()
            .cloned()
            .map(|component| component as Rc<RefCell<dyn Component>>)
    }

    fn despawn(&mut self, entity: usize) {
        self.components[entity] = None;
    }

    fn add_entity(&mut self) {
        self.components.push(None);
    }

    fn contained_type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }
}

/// This is the actual backend that stores all the entities and resources in the world.
#[derive(Default)]
pub struct Storage {
    /// All of the `Archetype`s that make up the `World`. Archetypes store the components that make
    /// up entities.
    pub archetypes: HashMap<TypeId, Rc<RefCell<dyn Archetype>>, TypeHasherBuilder>,
    /// All of the `Resource`s stored in the `World`. Each resource is stored by its type, so
    /// there can't be two resources of the same type.
    pub resources: HashMap<TypeId, Rc<RefCell<dyn Any>>, TypeHasherBuilder>,
    /// The number of entities that have existed in the world. This number is never decremented,
    /// even when entities are despawned; it does not track the number of entities currently in
    /// the world, but rather how many entities have existed, and what the next entity's ID
    /// would be.
    pub num_entities: usize,
}
impl Storage {
    /// Get the archetype for a particular component, by that component's `TypeId`.
    pub fn get_archetype(&self, id: TypeId) -> Option<Rc<RefCell<dyn Archetype>>> {
        self.archetypes.get(&id).cloned()
    }

    /// Set a component for an entity. This will overwrite an existing component, if there
    /// is one.
    pub fn insert_component(&mut self, entity: usize, component: impl Component) {
        self.insert_component_boxed(entity, Box::new(component))
    }
    /// Insert a component that's been boxed. This will overwrite an existing component, if
    /// there is one.
    pub fn insert_component_boxed(&mut self, entity: usize, component: Box<dyn Component>) {
        self.get_archetype((*component).type_id())
            .expect("TODO: Error handling. Archetype for component didn't exist when trying to insert component.")
            .deref()
            .borrow_mut()
            .set(entity, component)
    }

    /// Ensure the storage has an archetype for a particular component.
    pub fn prep_for<C: Component>(&mut self) {
        self.archetypes.entry(TypeId::of::<C>()).or_insert_with(|| {
            Rc::new(RefCell::new(WorldArchetype::<C>::new_with_size(
                self.num_entities,
            )))
        });
    }

    /// Insert a resouce into the world. This will overwrite a resource of the same type,
    /// if one already exists.
    pub fn insert_resource(&mut self, resource: impl Any) {
        self.resources
            .insert(resource.type_id(), Rc::new(RefCell::new(resource)) as _);
    }
    /// Get a resource by its `TypeId`.
    pub fn get_resource(&self, id: TypeId) -> Option<Rc<RefCell<dyn Any>>> {
        self.resources.get(&id).cloned()
    }

    /// Spawn a new entity and return its ID.
    pub fn spawn(&mut self) -> usize {
        for archetype in self.archetypes.values() {
            archetype.borrow_mut().add_entity()
        }

        self.num_entities += 1;
        self.num_entities - 1
    }
    /// Despawn an entity by its ID.
    pub fn despawn(&mut self, entity: usize) {
        self.archetypes
            .values()
            .for_each(|archetype| archetype.deref().borrow_mut().despawn(entity))
    }
}

/// The SECS storage uses `HashMap`s to map `TypeId`s to archetypes and resources.
/// This causes `TypeId`s to get hashed as keys; however, `TypeId`s are already hashes,
/// so that hash is entirely pointless and just wastes CPU cycles. This
/// "hasher" just returns the lower-64 bits of the `TypeId` to be used as the key,
/// instead of hashing that value. This also means a hashing dependency isn't needed.
///
/// This hasher is *only* meant to be used in `TypeId`s and *will* break for other
/// types.
pub struct TypeHasher(pub u64);

impl Hasher for TypeHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

/// A type that implements `BuildHasher` is necessary, for some reason. This is a
/// zero-size type that just creates a `TypeHasher` storing a 0.
#[derive(Default)]
pub struct TypeHasherBuilder;
impl BuildHasher for TypeHasherBuilder {
    type Hasher = TypeHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TypeHasher(0)
    }
}
