# SECS
The **S**mol **ECS** crate.

![Unsafe Blocks:](https://img.shields.io/badge/Unsafe%20Blocks-0-green)
![External Dependencies:](https://img.shields.io/badge/External%20Dependencies-0-green)
![Lines of Code](https://img.shields.io/tokei/lines/github/bright-shard/secs?label=Lines%20of%20Code)

SECS is an extremely small, but feature-complete, ECS library. To my knowledge, it's the first ECS library
to have no unsafe blocks, and the smallest feature-complete ECS library.

It has everything you'd expect - entities, components, and systems - without any of the bloat.
This makes SECS' source code easier to read than other ECS crates, and means it compiles faster than other crates.

# Entities

SECS comes with an `EntityBuilder` that can be used to make entities. For example:

```rs
// Make a new EntityBuilder
let player = EntityBuilder::new()
    // Add components: This can be any type that implements Component
    .add_component(Health::new(10))
    // The entity won't be inserted into the world until .build() is called
    // This returns the entity's id, in case you need it later
    .build();
```

# Components

SECS has a derivable `Component` trait for making components.

```rs
// A component for tracking an entity's health
#[derive(Component)]
pub struct Health(u8);

// Make some nice functions for it
impl Health {
    pub fn new(starting_health: u8) -> Self {
        Self(starting_health)
    }
    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
```

# Systems

Systems can query components and get global resources. They can only accept queries and resources, and must borrow them
(mutably or immutably, whichever you need).

```rs
fn remove_dead()
```
