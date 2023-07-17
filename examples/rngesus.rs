use secs::prelude::*;

fn main() {
    let mut world = World::default();

    let count: u8 = rand::random();
    println!("Generating {count} initial entities...\n\n");
    for _ in 0..count {
        make_entity().build(&mut world);
    }
    println!("\n\n");

    world.add_system(dmg);
    world.add_system(remove_dead);
    world.add_system(check_all_entities_dead);
    world.run();
}

// Entities

fn make_entity() -> EntityBuilder {
    let mut life = rand::random();
    if life < 0 {
        life += 1;
        life *= -1;
    }
    println!("Spawning entity with {life} life.");
    EntityBuilder::new().add_component(Life(life))
}

// Components

#[derive(Component)]
struct Life(i8);

// Systems

fn dmg(query: &mut Query<Life>) {
    let mut dmg: i8 = rand::random();
    if dmg < 0 {
        dmg += 1;
        dmg *= -1;
    }
    println!("Dealing {dmg} damage this round.");
    for entity in 0..query.len() {
        if let Some(life) = query.get_mut(entity) {
            life.0 -= dmg;
        }
    }
}

fn remove_dead(query: &Query<Life>, cmds: &mut CommandQueue) {
    for entity in 0..query.len() {
        if let Some(life) = query.get(entity) {
            if life.0 <= 0 {
                println!("Entity {entity} died! Despawning...");
                cmds.despawn(entity);
            }
        }
    }
}

fn check_all_entities_dead(query: &Query<Life>, cmds: &mut CommandQueue) {
    if query.get_somes().len() == 0 {
        println!("All entities have died, exiting run loop...");
        cmds.push(Command::ExitRunLoop);
    }
}
