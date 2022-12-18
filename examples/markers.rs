use bevy::prelude::{App, Commands, Component, Query};
use bevy_ecs_markers::{Marker, MarkerData, MarkerMut};
use bevy_ecs_markers_macros::EntityMarker;

#[derive(EntityMarker, Hash, PartialEq, Eq)]
enum Players {
    Red,
    Blue,
}

#[derive(EntityMarker, Hash, PartialEq, Eq)]
struct CurrentPlayer;

#[derive(Component)]
struct Player(u32);

fn main() {
    App::new()
        .init_resource::<MarkerData<Players>>()
        .init_resource::<MarkerData<CurrentPlayer>>()
        .add_startup_system(setup)
        .add_system(get_red_player)
        .add_system(get_current_player)
        .run();
}

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
