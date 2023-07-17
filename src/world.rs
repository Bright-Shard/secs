use {
    crate::{
        archetype::{AnonymousArchetype, Archetype},
        command_queue::Command,
        entity::Component,
        resource::{AnonymousWorldResource, Resource},
        system::{IntoSystem, System, Systems},
    },
    std::{
        any::{Any, TypeId},
        collections::hash_map::{Entry, HashMap},
        mem::swap,
    },
};

pub struct World {
    archetypes: Vec<Option<Box<dyn AnonymousArchetype>>>,
    archetype_map: HashMap<TypeId, usize>,
    entities: usize,
    resources: Vec<Option<Box<dyn AnonymousWorldResource>>>,
    resource_map: HashMap<TypeId, usize>,
    systems: Option<Systems>,
    commands: Vec<Command>,
    exit_runloop: bool,
}
impl Default for World {
    fn default() -> Self {
        Self {
            archetypes: Vec::new(),
            archetype_map: HashMap::new(),
            entities: 0,
            resources: Vec::new(),
            resource_map: HashMap::new(),
            systems: Some(Systems::default()),
            commands: Vec::new(),
            exit_runloop: false,
        }
    }
}
impl World {
    pub fn new_entity(&mut self) -> usize {
        let result = self.entities;
        self.entities += 1;
        result
    }

    pub fn query<C: Component + 'static>(&self) -> Option<&Archetype<C>> {
        let id = self.archetype_map.get(&TypeId::of::<C>())?;
        if let Some(archetype) = (*self.archetypes).get(*id)? {
            archetype.as_any().downcast_ref::<Archetype<C>>()
        } else {
            None
        }
    }
    pub fn query_mut<C: Component + 'static>(&mut self) -> Option<&mut Archetype<C>> {
        let id = self.archetype_map.get(&TypeId::of::<C>())?;
        if let Some(archetype) = (*self.archetypes).get_mut(*id)? {
            archetype.as_any_mut().downcast_mut::<Archetype<C>>()
        } else {
            None
        }
    }

    pub fn query_from_entity<C: Component + 'static>(&self, entity: usize) -> Option<&C> {
        match self.query::<C>() {
            None => None,
            Some(archetype) => archetype.get(entity),
        }
    }
    pub fn query_from_entity_mut<C: Component + 'static>(
        &mut self,
        entity: usize,
    ) -> Option<&mut C> {
        match self.query_mut::<C>() {
            None => None,
            Some(archetype) => archetype.get_mut(entity),
        }
    }

    pub fn prep_archetype<C: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<C>();
        if let Entry::Vacant(entry) = self.archetype_map.entry(type_id) {
            self.archetypes
                .push(Some(Box::new(Archetype::<C>::new(self.entities))));
            entry.insert(self.archetypes.len() - 1);
        }
    }

    pub fn insert_component<C: Component + 'static>(&mut self, entity: usize, component: C) {
        // Ensure we have an archetype for this component type already
        self.prep_archetype::<C>();
        let archetype = self.query_mut::<C>().unwrap();
        archetype.set(entity, Some(component));
    }

    #[allow(clippy::result_unit_err)]
    pub fn insert_component_unchecked(
        &mut self,
        entity: usize,
        component: Box<dyn Any>,
    ) -> Result<(), ()> {
        let type_id = (*component).type_id();
        let id = self.archetype_map.get(&type_id).ok_or(())?;
        let archetype = (*self.archetypes).get_mut(*id).ok_or(())?;
        archetype
            .as_mut()
            .ok_or(())?
            .set_unchecked(entity, component)
    }

    pub fn take_resource<D: 'static>(&mut self) -> Option<Resource<D>> {
        let id = self.resource_map.get(&TypeId::of::<D>())?;
        let borrowed_resource = self.resources.get_mut(*id)?;
        let mut resource = None;
        swap(borrowed_resource, &mut resource);

        resource.map(|resource| *resource.as_any_owned().downcast::<Resource<D>>().unwrap())
    }
    pub fn take_archetype<C: Component + 'static>(&mut self) -> Option<Archetype<C>> {
        let id = self.archetype_map.get(&TypeId::of::<C>())?;
        let borrowed_archetype = self.archetypes.get_mut(*id)?;
        let mut archetype = None;
        swap(borrowed_archetype, &mut archetype);

        archetype.map(|archetype| *archetype.as_any_owned().downcast::<Archetype<C>>().unwrap())
    }

    pub fn return_resource<D: 'static>(&mut self, resource: Resource<D>) {
        let id = self.resource_map.get(&TypeId::of::<D>()).unwrap();
        self.resources[*id] = Some(Box::new(resource) as Box<dyn AnonymousWorldResource>);
    }
    pub fn return_archetype<C: Component + 'static>(&mut self, archetype: Archetype<C>) {
        let id = self.archetype_map.get(&TypeId::of::<C>()).unwrap();
        self.archetypes[*id] = Some(Box::new(archetype) as Box<dyn AnonymousArchetype>);
    }

    pub fn add_system<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
        self.systems.as_mut().unwrap().push(system);
    }
    pub fn add_resource<D: 'static>(&mut self, data: D) {
        let id = TypeId::of::<D>();
        if self.resource_map.get(&id).is_none() {
            self.resources.push(Some(
                Box::new(Resource(data)) as Box<dyn AnonymousWorldResource>
            ));
            self.resource_map.insert(id, self.resources.len() - 1);
        }
    }

    pub fn run_once(&mut self) {
        let mut systems = None;
        swap(&mut self.systems, &mut systems);
        systems.as_mut().unwrap().run(self);
        swap(&mut systems, &mut self.systems);
    }
    pub fn run(&mut self) {
        let mut systems = None;
        swap(&mut self.systems, &mut systems);
        let systems = systems.unwrap();

        while !self.exit_runloop {
            systems.run(self);
        }
    }

    pub fn despawn(&mut self, entity: usize) {
        for archetype in &mut self.archetypes {
            archetype.as_mut().unwrap().despawn(entity);
        }
    }

    pub fn apply_commands(&mut self) {
        let mut commands = Vec::new();
        std::mem::swap(&mut self.commands, &mut commands);

        for command in commands {
            match command {
                Command::SpawnEntity(builder) => {
                    builder.build(self);
                }
                Command::DespawnEntity(id) => self.despawn(id),
                Command::ExitRunLoop => self.exit_runloop = true,
            };
        }
    }
    pub fn add_commands(&mut self, commands: Vec<Command>) {
        self.commands.extend(commands.into_iter());
    }
}
