use crate::{entity::EntityBuilder, system::WorldData, world::World};

#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Command>,
}
pub enum Command {
    SpawnEntity(EntityBuilder),
    DespawnEntity(usize),
    ExitRunLoop,
}

impl CommandQueue {
    pub fn spawn(&mut self, entity: EntityBuilder) {
        self.commands.push(Command::SpawnEntity(entity));
    }
    pub fn despawn(&mut self, entity: usize) {
        self.commands.push(Command::DespawnEntity(entity));
    }

    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }
}

impl WorldData for CommandQueue {
    fn take(_: &mut World) -> Self {
        Self::default()
    }
    fn release(self, world: &mut World) {
        world.add_commands(self.commands);
    }
}
