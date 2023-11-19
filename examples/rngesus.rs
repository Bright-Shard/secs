// This example demos systems, components, and commands in SECS. The README has additional context
// that might help in understanding this example - go check that out first!

use secs::prelude::*;

fn main() {
    let mut world = World::default();

    // Generate a random amount of entities in the world
    let count: u8 = rand::random();
    println!("Generating {count} initial entities...\n\n");
    for _ in 0..count {
        make_entity(&mut world);
    }
    println!("\n\n");

    world.add_system(dmg);
    world.add_system(remove_dead);
    world.add_system(check_all_entities_dead);
    world.run();
}

// Entities

fn make_entity(world: &mut World) {
    // Each entity gets a random amount of life
    let life: u8 = rand::random();
    let entity = world.spawn(Life(life.try_into().unwrap_or(i8::MAX)));
    println!("Spawned entity {entity} with {life} life.");
}

// Components

#[derive(Component)]
struct Life(i8);

// Systems

// Damages entities randomly
fn dmg(query: &Query<&mut Life>) {
    // All the entities take a random amount of damage each round.
    let dmg: u8 = rand::random();
    println!("\nDealing {dmg} damage this round.");

    for mut life_component in query {
        life_component.0 -= dmg.try_into().unwrap_or(i8::MAX);
    }
}

// Removes dead entities
fn remove_dead(query: &Query<&Life>, cmds: &mut CommandQueue) {
    // To despawn entities, we need their entity ID. To do this, we'll use a special iterator in `Query`
    // that gives us the component from the query and the entity that component belongs to.
    for (entity, life) in query.iter_with_entity() {
        if life.0 <= 0 {
            println!("Entity {entity} died! Despawning...");
            cmds.despawn(entity);
        }
    }
}

fn check_all_entities_dead(query: &Query<&Life>, cmds: &mut CommandQueue) {
    if query.is_empty() {
        println!("All entities have died, exiting run loop...");
        cmds.push(Command::ExitRunLoop);
    }
}
