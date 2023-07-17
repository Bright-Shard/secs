use secs::prelude::*;

fn main() {
    let mut world = World::default();
    world.add_system(my_system);
    world.add_resource(0);
    for _ in 0..10 {
        add_entity(&mut world);
    }
    world.run();
}

#[derive(Debug, Component)]
struct MyComponent {}

fn add_entity(world: &mut World) {
    EntityBuilder::new()
        .add_component(MyComponent {})
        .build(world);
}

fn my_system(resource: &Resource<i32>, query: &Query<MyComponent>) {
    println!("System running!");
    println!("Got the resource: {}", resource.get());
    println!("Got the query: {:?}", query);
}
