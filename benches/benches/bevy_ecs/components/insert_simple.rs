// use bevy_ecs::prelude::*;
use glam::*;
use secs::prelude::*;

#[derive(Component, Copy, Clone)]
struct Transform(Mat4);

#[derive(Component, Copy, Clone)]
struct Position(Vec3);

#[derive(Component, Copy, Clone)]
struct Rotation(Vec3);

#[derive(Component, Copy, Clone)]
struct Velocity(Vec3);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        // let mut world = World::new();
        let mut world = World::default();
        world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Mat4::from_scale(Vec3::ONE)),
                Position(Vec3::X),
                Rotation(Vec3::X),
                Velocity(Vec3::X),
            )
        }));
    }
}
