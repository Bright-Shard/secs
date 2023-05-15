use secs::prelude::*;

fn main() {
    let mut world = World::default();
    world.add_system(my_system);
    world.add_resource(0);
    world.run();
}

fn my_system(resource: &Resource<i32>) {
    println!("System running!");
    println!("Got the resource: {}", resource.get());
}
