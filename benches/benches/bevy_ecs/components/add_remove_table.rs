// use bevy_ecs::prelude::*;
use secs::prelude::*;

#[derive(Component)]
struct A(f32);
#[derive(Component)]
struct B(f32);

// pub struct Benchmark(World, Vec<Entity>);
pub struct Benchmark(World, Vec<usize>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut entities = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            // entities.push(world.spawn(A(0.0)).id());
            entities.push(world.spawn(A(0.0)));
        }

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            // self.0.entity_mut(*entity).insert(B(0.0));
            self.0.insert_components(*entity, B(0.0));
        }

        for entity in &self.1 {
            // self.0.entity_mut(*entity).remove::<B>();
            self.0.remove_component::<B>(*entity);
        }
    }
}
