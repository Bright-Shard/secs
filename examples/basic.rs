// This example demos resources, systems, and entities being stored in the world.

use secs::prelude::*;

fn main() {
    let mut world = World::default();

    world.add_system(my_system);

    // Resources are stored by type. In this case, we're storing an i32, so if we get a
    // `Resource<i32>` in a system, it'll get us this value.
    // Although this demo doesn't show it, you can, of course, borrow resources mutably and modify
    // them in systems. The world will store the new value.
    world.insert_resource(42);

    // Entities own their components, so even though we're making entities that store the same data
    // here, we can't reuse the `EntityBuilder`. Each entity must have its own components.
    for id in 0..10 {
        world.spawn(MyComponent { id });
    }

    // Just to show the printouts from the system below
    world.run_once();
}

#[derive(Debug, Component)]
struct MyComponent {
    id: u8,
}

fn my_system(resource: &Resource<i32>, query: &Query<&MyComponent>) {
    println!("System running!");
    println!("Got the resource: {}", resource.get());

    for component in query.iter() {
        println!("{}", component.id)
    }
}
