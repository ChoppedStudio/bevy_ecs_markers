# Bevy ECS Markers

[![Crates.io](https://img.shields.io/crates/v/bevy_ecs_markers.svg)](https://crates.io/crates/bevy_ecs_markers)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ChoppedStudio/bevy_ecs_markers#license)

Adds the support for marking entites and fetching them in queries

## Example

View the whole example [here](examples/markers.rs)

```rust
#[derive(EntityMarker)]
enum Players {
    Red,
    Blue,
}

#[derive(EntityMarker)]
struct CurrentPlayer;

#[derive(Component)]
struct Player(u32);

fn setup(
    mut commands: Commands,
    mut markers: MarkerMut<Players>,
    mut current: MarkerMut<CurrentPlayer>,
) {
    let red = commands.spawn(Player(12)).id();
    markers[Players::Red] = red;

    let blue = commands.spawn(Player(7)).id();
    markers[Players::Blue] = blue;

    *current = blue;
}

fn get_red_player(mut query: Query<&mut Player>, markers: Marker<Players>) {
    if let Ok(mut player) = query.get_mut(markers[Players::Red]) {
        player.0 = 15;
    }
}

fn get_current_player(mut query: Query<&mut Player>, current: Marker<CurrentPlayer>) {
    if let Ok(mut player) = query.get_mut(*current) {
        player.0 = 2;
    }
}
```

## License

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.