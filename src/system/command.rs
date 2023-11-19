use {
    crate::{_crate_prelude::*, entity::Bundle, system::WorldData},
    core::ops::{Deref, DerefMut},
};

/// This is what systems will use to queue commands for the world. The commands will be applied
/// once the system finishes running - *not* while it's running.
///
/// Commands are applied after the system finishes running to avoid conflicting data. For example,
/// if a system tried to mutably query a component and insert an entity at the same time, it would
/// break Rust's mutability rules and crash. To avoid this, after a System finishes running, the
/// `CommandQueue` is applied and commands take effect.
#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Command>,
}

/// The actual Commands that can be applied to a World.
pub enum Command {
    /// Finish building an EntityBuilder.
    SpawnEntity(Box<dyn Bundle>),
    /// Remove an Entity from the World by its ID.
    DespawnEntity(usize),
    /// Exits the loop started by `World.run()`.
    ExitRunLoop,
}

impl CommandQueue {
    /// Spawns an entity into the world.
    pub fn spawn(&mut self, entity: impl Bundle + 'static) {
        self.commands.push(Command::SpawnEntity(Box::new(entity)));
    }

    /// Despawns an Entity by its ID.
    pub fn despawn(&mut self, entity: usize) {
        self.commands.push(Command::DespawnEntity(entity));
    }

    /// Exits the loop started by `World.run()`.
    pub fn exit_run_loop(&mut self) {
        self.commands.push(Command::ExitRunLoop);
    }
}
impl Deref for CommandQueue {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}
impl DerefMut for CommandQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

/// Allows the `CommandQueue` struct to be used as a system parameter.
impl WorldData for CommandQueue {
    fn take(_: &mut World) -> Self {
        Self::default()
    }

    fn release(self, world: &mut World) {
        world.apply_commands(self.commands);
    }
}
