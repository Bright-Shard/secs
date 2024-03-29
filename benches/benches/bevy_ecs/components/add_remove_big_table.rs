use glam::*;
// use bevy_ecs::prelude::*;
use secs::prelude::*;

#[derive(Component, Copy, Clone)]
struct A(Mat4);
#[derive(Component, Copy, Clone)]
struct B(Mat4);

#[derive(Component, Copy, Clone)]
struct C(Mat4);
#[derive(Component, Copy, Clone)]
struct D(Mat4);

#[derive(Component, Copy, Clone)]
struct E(Mat4);

#[derive(Component, Copy, Clone)]
struct F(Mat4);
// pub struct Benchmark(World, Vec<Entity>);
pub struct Benchmark(World, Vec<usize>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut entities = Vec::with_capacity(10_000);
        for _ in 0..10_000 {
            entities.push(
                world.spawn((
                    A(Mat4::from_scale(Vec3::ONE)),
                    B(Mat4::from_scale(Vec3::ONE)),
                    C(Mat4::from_scale(Vec3::ONE)),
                    D(Mat4::from_scale(Vec3::ONE)),
                    E(Mat4::from_scale(Vec3::ONE)),
                )), // .id(),
            );
        }

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity in &self.1 {
            // self.0
            //     .entity_mut(*entity)
            //     .insert(F(Mat4::from_scale(Vec3::ONE)));

            self.0
                .insert_components(*entity, F(Mat4::from_scale(Vec3::ONE)));
        }

        for entity in &self.1 {
            // self.0.entity_mut(*entity).remove::<F>();
            self.0.remove_component::<F>(*entity);
        }
    }
}
