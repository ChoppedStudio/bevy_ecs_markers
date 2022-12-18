# Bevy ECS Markers

Adds the support for marking entites and fetching them in queries

## Example

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

fn setup(mut commands: Commands, mut markers: MarkerMut<Players>, mut current: MarkerMut<CurrentPlayer>) {
    let red = commands.spawn(Player(12)).id();
    markers[Players::Red] = red;

    let blue = commands.spawn(Player(7)).id();
    markers[Players::Blue] = blue;

    *current = blue;
}

fn get_red_player(mut query: Query<&mut Player>, markers: Marker<Players>) {
    let mut player = query.single_mut(markers[Players::Red]);
    player.0 = 15;
}

fn get_current_player(mut query: Query<&mut Player>, current: Marker<CurrentPlayer>) {
    let mut player = query.single_mut(*current);
    player.0 = 2;
}
```