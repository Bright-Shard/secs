pub mod storage;
pub use storage::*;

use {
    crate::{
        _crate_prelude::*,
        entity::Bundle,
        system::{command::Command, IntoSystem, System, Systems},
    },
    alloc::rc::Rc,
    core::cell::RefCell,
};

/// The ECS World, which holds all the data in the program.
pub struct World {
    /// Where all of the entities and resources in the World are actually stored.
    pub storage: Storage,
    /// All of the Systems registered in the World.
    pub systems: Rc<RefCell<Systems>>,
    /// A flag for the run loop started in `World::run()`. When true, the loop breaks. The
    /// `ExitRunLoop` command sets this to true.
    pub exit_run_loop: bool,
}
impl Default for World {
    fn default() -> Self {
        Self {
            storage: Storage::default(),
            systems: Rc::new(RefCell::new(Systems::default())),
            exit_run_loop: false,
        }
    }
}
impl World {
    /// Spawns an entity into the World. Returns its ID.
    #[inline]
    pub fn spawn(&mut self, components: impl Bundle) -> usize {
        let entity = self.storage.spawn();
        components.components().into_iter().for_each(|component| {
            component.prep_storage(&mut self.storage);
            self.storage.insert_component_boxed(entity, component);
        });
        entity
    }
    /// Spawns an entity, whose components are boxed, into the World. Returns its ID.
    #[inline]
    pub fn spawn_boxed(&mut self, components: Box<dyn Bundle>) -> usize {
        let entity = self.storage.spawn();
        components
            .components_from_box()
            .into_iter()
            .for_each(|component| {
                component.prep_storage(&mut self.storage);
                self.storage.insert_component_boxed(entity, component);
            });
        entity
    }
    /// Spawns an entity into the World with no components. Returns the entity's ID.
    #[inline]
    pub fn spawn_empty(&mut self) -> usize {
        self.storage.spawn()
    }

    /// Registers a resource in the world. This will overwrite any existing resources
    /// of the same type.
    #[inline]
    pub fn insert_resource(&mut self, resource: impl Any + 'static) {
        self.storage.insert_resource(resource);
    }
    /// Insert one or more components into an entity.
    #[inline]
    pub fn insert_components(&mut self, entity: usize, components: impl Bundle) {
        components.components().into_iter().for_each(|component| {
            component.prep_storage(&mut self.storage);
            self.storage.insert_component_boxed(entity, component);
        });
    }
    /// Remove a component from an entity.
    #[inline]
    pub fn remove_component<C: Component>(&mut self, entity: usize) {
        self.storage
            .get_archetype(TypeId::of::<C>())
            .unwrap()
            .borrow_mut()
            .despawn(entity);
    }
    /// Remove a component from an entity, by the component's `TypeId`.
    #[inline]
    pub fn remove_component_by_id(&mut self, entity: usize, component: TypeId) {
        self.storage
            .get_archetype(component)
            .unwrap()
            .borrow_mut()
            .despawn(entity);
    }

    /// Register a System in the World.
    #[inline]
    pub fn add_system<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
        self.systems.borrow_mut().push(system);
    }

    /// Runs all of the World's Systems once. This will run even if the `ExitRunLoop` command has been
    /// used.
    #[inline]
    pub fn run_once(&mut self) {
        self.systems.clone().borrow().run(self);
    }
    /// Runs all of the World's Systems in a loop. The loop can be broken with the `ExitRunLoop`
    /// command; however, calling this method again after exiting will restart the loop until
    /// `ExitRunLoop` is called again.
    pub fn run(&mut self) {
        self.exit_run_loop = false;
        let systems = self.systems.clone();
        let systems = systems.borrow();

        while !self.exit_run_loop {
            systems.run(self);
        }
    }

    /// Applies changes from a command queue to the world.
    pub fn apply_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::SpawnEntity(bundle) => {
                    self.spawn_boxed(bundle);
                }
                Command::DespawnEntity(id) => self.storage.despawn(id),
                Command::ExitRunLoop => self.exit_run_loop = true,
            };
        }
    }
}
