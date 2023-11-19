# SECS
The **S**mol **ECS** crate. Or, perhaps, the **S**afe **ECS** crate. Or, perhaps, the **S**mart **ECS** crate.
Or, perhaps, the ECS crate with a funny name. 

![Unsafe Blocks](https://img.shields.io/badge/Unsafe%20Blocks-0-green)
![External Dependencies](https://img.shields.io/badge/External%20Dependencies-1-green)
![Lines of Code](https://img.shields.io/tokei/lines/github/bright-shard/secs?label=Lines%20of%20Code)

SECS is an extremely small, but feature-complete, ECS library. To my knowledge, it's the first ECS library
to be written entirely in safe Rust code, and one of the smallest ECS implementations in existance. It
aims to be simpler than other ECS implementations, so other people can read it and understand how ECS works
internally, and so the codebase is more manageable. SECS also aims to have near feature-parity with
Bevy's ECS implementation, besides multithreading.

# ECS Basics

Here's how ECS works in SECS:

## The World

SECS tracks everything in the ECS world, just as you'd expect. You can make one by calling `World::default()`.

```rs
use secs::prelude::*;

let mut world = World::default();
```

## Entities

Entities are groups of components, so SECS represents them with tuples (these are just like
Bevy's bundles). You can spawn entities with `World.spawn()`, which takes either a single
component or a tuple of components:

```rs
// One component...
world.spawn(Health(20));

// Or many!
world.spawn((Health(10), Strength(20));
```

## Components

SECS has a derivable `Component` trait for making components.

```rs
// A component for tracking an entity's health.
#[derive(Component)]
pub struct Health(pub u8);

// A component for tracking an entity's strength.
#[derive(Component)]
pub struct Health(pub u8);

// You can add arbitrary fields and methods to components, and use them later in Systems.
impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
```

## Systems

Systems can access data from the world with system parameters. Currently, the system parameters are as follows:

- `Query<Components>`: Allows you to get all entities that have `<Components>` as components, and modify those components.
`Query` respects Rust's mutability: You must borrow components, either as `&Component` or `&mut Component`, but can only modify
ones that are mutably borrowed. You can mix and match mutable components - for example, `Query<(&Strength, &mut Health)> is valid,
but will only let you modify the health component.
- `CommandQueue`: Allows a system to work with `Command`s, which can modify the world. Commands can currently spawn and despawn
entities, and exit the run loop (if you used the world's run loop, which just infinitely calls systems). Commands are only applied
after the system finishes running, to prevent the system and a command from trying to mutably borrow the same data.
- `Resource<ResourceType>`: Allows you to access resources, which act like global variables. There can only be one resource of
each type (`i32`, `ACustomStruct`, etc), but otherwise there can be unlimited resources.

All parameters respect mutability rules. You can borrow (mutably or immutably) parameters, but not take ownership of them, since
their data is owned by the `World`. You cannot modify immutably-borrowed parameters.

Here's some example systems:

```rs
// Damages entities randomly
fn dmg(query: &Query<&mut Life>) {
    // All the entities take a random amount of damage each round.
    let dmg: u8 = rand::random();
    println!("\nDealing {dmg} damage this round.");

    for mut life_component in query {
        // Notice: we can mutate life, even though the query was borrowed immutably. This is because
        // the life component itself was borrowed mutably; if it weren't, this wouldn't work.
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
```

To register a system, just call `World.add_system()`:

```rs
world.add_system(rngesus);
world.add_system(remove_dead);
```

To run systems, call `World.run_once()` or `World.run()`. `run` will start a loop and run the systems until a system breaks the loop;
`run_once` will simply execute all systems one time. A system can break a `run` loop with the `ExitRunLoop` command.

# Issues and Limitations

- SECS is currently single-threaded - it does not support multithreaded access to the `World`.
- There's no way to use a custom storage method for the world. I attempted to add this, but it made
the code extremely messy. I may attempt to add it again in the future.
- SECS aims for feature-parity with Bevy's ECS, besides multithreading, but is missing features:
    - Query filters (`Has`, `Option`, etc)
    - Change detection/state
    - Events
    - There are probably more - open an issue if so!
